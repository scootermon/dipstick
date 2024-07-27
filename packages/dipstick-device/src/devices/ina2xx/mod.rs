use std::time::SystemTime;

use dipstick_core::Core;
use dipstick_proto::device::v1::{DeviceSpecVariant, Ina2xxSpec};
use futures::future::BoxFuture;
use futures::FutureExt;
use tokio::time;
use tonic::{Result, Status};

use self::model::Model;
use self::spec::Spec;
use self::transport::Transport;
use super::DeviceVariant;
use crate::Device;

mod model;
mod register;
mod spec;
mod transport;

pub struct Ina2xx {
    transport: Transport,
    spec: Spec,
    state: tokio::sync::RwLock<State>,
}

struct State {
    model: Model,
    current_lsb: f32,
    shunt_cal: u16,
}

impl Ina2xx {
    pub fn new(core: &Core, device: &Device, mut spec: Ina2xxSpec) -> Result<Self> {
        let Some(transport_spec) = spec.ina2xx_transport_spec.take() else {
            return Err(Status::invalid_argument("missing ina2xx transport spec"));
        };

        let transport = Transport::new(core, device, transport_spec)?;
        let spec = Spec::from_proto(spec)?;
        let state = tokio::sync::RwLock::new(State {
            // the correct values will be set during setup
            model: Model::Ina228,
            current_lsb: 0.0,
            shunt_cal: 0,
        });
        Ok(Self {
            transport,
            spec,
            state,
        })
    }

    // setup

    async fn setup(&self) -> Result<()> {
        let mut state = self.state.write().await;
        tracing::debug!("starting setup");
        self.reset_config().await?;
        time::sleep(time::Duration::from_millis(2)).await;
        self.check_device_model(&mut state).await?;
        time::sleep(time::Duration::from_millis(1)).await;
        self.configure_adc_range().await?;
        time::sleep(time::Duration::from_millis(1)).await;
        self.configure_adc().await?;
        time::sleep(time::Duration::from_millis(1)).await;
        self.configure_shunt(&mut state).await?;
        time::sleep(time::Duration::from_millis(1)).await;
        self.configure_shunt_tempco(&state).await?;
        time::sleep(time::Duration::from_millis(1)).await;
        tracing::debug!("setup complete");
        Ok(())
    }

    async fn reset_config(&self) -> Result<()> {
        tracing::trace!("reset");
        self.transport
            .write_register(register::Config {
                rst: true,
                ..Default::default()
            })
            .await?;
        Ok(())
    }

    async fn check_device_model(&self, state: &mut State) -> Result<()> {
        tracing::trace!("check device model");
        let manufacturer_id: register::ManufacturerId = self.transport.read_register().await?;
        let device_id: register::DeviceId = self.transport.read_register().await?;

        tracing::debug!(
            manufacturer_id.manf_id,
            device_id.die_id,
            device_id.rev_id,
            "device model"
        );
        state.model = Model::from_registers(manufacturer_id.manf_id, device_id.die_id)?;
        Ok(())
    }

    async fn configure_adc_range(&self) -> Result<()> {
        tracing::trace!(adc_range = ?self.spec.adc_range, "configure adc range");
        let mut cfg: register::Config = self.transport.read_register().await?;
        cfg.adc_range = self.spec.adc_range;
        self.transport.write_register(cfg).await
    }

    async fn configure_adc(&self) -> Result<()> {
        let adc_cfg = register::AdcConfig {
            mode: 0x0F, // Fh = Continuous bus voltage, shunt voltage and temperature
            vbus_ct: self.spec.bus_voltage_conversion_time,
            vsh_ct: self.spec.shunt_voltage_conversion_time,
            vt_ct: self.spec.temperature_conversion_time,
            avg: self.spec.avg_samples,
        };
        tracing::trace!(?adc_cfg, "configure adc");
        self.transport.write_register(adc_cfg).await
    }

    async fn configure_shunt(&self, state: &mut State) -> Result<()> {
        // model must be known at this point
        let calib = state.model.calibration();

        state.current_lsb =
            self.spec.max_current_a * 2.0f32.powi(i32::from(calib.current_lsb_scale_factor));
        state.shunt_cal =
            (calib.shunt_cal_scale * state.current_lsb * self.spec.shunt_resistance_ohm) as u16;
        if matches!(self.spec.adc_range, register::AdcRange::Range1) {
            state.shunt_cal *= 4;
        }

        if state.shunt_cal & 0x8000 != 0 {
            tracing::warn!("shunt value too high");
        }
        state.shunt_cal &= 0x7FFF;
        let reg = register::ShuntCal {
            shunt_cal: state.shunt_cal,
        };
        tracing::debug!(reg.shunt_cal, "configure shunt");
        self.transport.write_register(reg).await
    }

    async fn configure_shunt_tempco(&self, state: &State) -> Result<()> {
        // Only for 228/229
        if !(matches!(state.model, Model::Ina228 | Model::Ina229)
            && self.spec.shunt_tempco_ppm_c > 0)
        {
            return Ok(());
        }
        let reg = register::ShuntTempco {
            temp_co: self.spec.shunt_tempco_ppm_c,
        };
        tracing::trace!(reg.temp_co, "configure shunt tempco");
        self.transport.write_register(reg).await
    }

    // reading

    async fn read_shunt_voltage_mv(&self) -> Result<f32> {
        let state = self.state.read().await;
        let calib = state.model.calibration();
        match state.model {
            Model::Ina228 | Model::Ina229 => {
                let reg: register::VShunt3 = self.transport.read_register().await?;
                let shunt_lsb = match self.spec.adc_range {
                    register::AdcRange::Range0 => calib.vshunt_lsb_range0,
                    register::AdcRange::Range1 => calib.vshunt_lsb_range1,
                };
                Ok(reg.vshunt as f32 * shunt_lsb)
            }
        }
    }

    async fn read_bus_voltage(&self) -> Result<f32> {
        let state = self.state.read().await;
        let calib = state.model.calibration();
        match state.model {
            Model::Ina228 | Model::Ina229 => {
                let reg: register::VBus3 = self.transport.read_register().await?;
                Ok(calib.vbus_lsb * reg.vbus as f32)
            }
        }
    }

    async fn read_die_temp_c(&self) -> Result<f32> {
        let state = self.state.read().await;
        let calib = state.model.calibration();
        match state.model {
            Model::Ina228 | Model::Ina229 => {
                let reg: register::DieTemp16 = self.transport.read_register().await?;
                Ok(calib.die_temp_lsb * reg.die_temp as f32)
            }
        }
    }

    async fn read_current_a(&self) -> Result<f32> {
        let state = self.state.read().await;
        match state.model {
            Model::Ina228 | Model::Ina229 => {
                let reg: register::Current3 = self.transport.read_register().await?;
                Ok(state.current_lsb * reg.current as f32)
            }
        }
    }

    async fn read_power_w(&self) -> Result<f32> {
        let state = self.state.read().await;
        let calib = state.model.calibration();
        match state.model {
            Model::Ina228 | Model::Ina229 => {
                let reg: register::Power3 = self.transport.read_register().await?;
                Ok(calib.power_coeff * state.current_lsb * reg.power as f32)
            }
        }
    }
}

impl DeviceVariant for Ina2xx {
    fn spec(&self) -> DeviceSpecVariant {
        let mut spec = self.spec.to_proto();
        spec.ina2xx_transport_spec = Some(self.transport.spec());
        DeviceSpecVariant::Ina2xx(spec)
    }

    fn start<'s: 'fut, 'fut>(&'s self, device: &'fut Device) -> BoxFuture<'fut, Result<()>> {
        async move {
            self.setup().await?;
            device.set_attr("setup_complete", true);
            Ok(())
        }
        .boxed()
    }

    fn stop<'s: 'fut, 'fut>(&'s self, _device: &'fut Device) -> BoxFuture<'fut, Result<()>> {
        async move { Ok(()) }.boxed()
    }

    fn call_action<'s: 'fut, 'fut>(
        &'s self,
        _device: &'fut Device,
        _action: &'fut str,
    ) -> BoxFuture<'fut, Result<()>> {
        async move { Err(Status::invalid_argument("unknown action")) }.boxed()
    }

    fn update<'s: 'fut, 'fut>(&'s self, device: &'fut Device) -> BoxFuture<'fut, Result<()>> {
        async move {
            let now = || dipstick_proto::wkt::Timestamp::from(SystemTime::now());

            match self.read_shunt_voltage_mv().await {
                Ok(v) => device.set_sensor_value("shunt_voltage", now(), v),
                Err(err) => tracing::error!(
                    err = &err as &dyn std::error::Error,
                    "failed to read shunt voltage"
                ),
            }
            match self.read_bus_voltage().await {
                Ok(v) => device.set_sensor_value("bus_voltage", now(), v),
                Err(err) => tracing::error!(
                    err = &err as &dyn std::error::Error,
                    "failed to read bus voltage"
                ),
            }
            match self.read_die_temp_c().await {
                Ok(v) => device.set_sensor_value("die_temp", now(), v),
                Err(err) => tracing::error!(
                    err = &err as &dyn std::error::Error,
                    "failed to read die temp"
                ),
            }
            match self.read_current_a().await {
                Ok(v) => device.set_sensor_value("current", now(), v),
                Err(err) => tracing::error!(
                    err = &err as &dyn std::error::Error,
                    "failed to read current"
                ),
            }
            match self.read_power_w().await {
                Ok(v) => device.set_sensor_value("power", now(), v),
                Err(err) => {
                    tracing::error!(err = &err as &dyn std::error::Error, "failed to read power")
                }
            }

            Ok(())
        }
        .boxed()
    }
}
