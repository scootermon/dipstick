use std::sync::Arc;

use dipstick_core::{Core, DependencyHandle, Entity, EntityKind, EntityMeta};
use dipstick_proto::stack::v1::{StackEntity, StackSpec, StackStatus};
use tonic::Result;

pub struct Stack {
    meta: EntityMeta,
    spec: StackSpec,
    _dependency_handles: DependencyHandles,
}

impl Stack {
    pub async fn new(core: &Core, meta: EntityMeta, mut spec: StackSpec) -> Result<Arc<Self>> {
        let StackSpec {
            can,
            device,
            gpio,
            shadow,
            spi,
            xcp,
        } = &mut spec;

        let mut dependency_handles = DependencyHandles::new();

        if let Some(spi) = spi {
            for spec in &mut spi.device {
                add_spi_device(
                    Context {
                        core,
                        meta: &meta,
                        dependency_handles: &mut dependency_handles,
                    },
                    spec,
                )
                .await?;
            }
        }

        if let Some(gpio) = gpio {
            for spec in &mut gpio.chip {
                add_gpio_chip(
                    Context {
                        core,
                        meta: &meta,
                        dependency_handles: &mut dependency_handles,
                    },
                    spec,
                )
                .await?;
            }
        }

        if let Some(can) = can {
            for spec in &mut can.bus {
                add_can_bus(
                    Context {
                        core,
                        meta: &meta,
                        dependency_handles: &mut dependency_handles,
                    },
                    spec,
                )
                .await?;
            }
        }

        if let Some(xcp) = xcp {
            for spec in &mut xcp.a2l {
                add_a2l(
                    Context {
                        core,
                        meta: &meta,
                        dependency_handles: &mut dependency_handles,
                    },
                    spec,
                )
                .await?;
            }

            for spec in &mut xcp.session {
                add_xcp_session(
                    Context {
                        core,
                        meta: &meta,
                        dependency_handles: &mut dependency_handles,
                    },
                    spec,
                )
                .await?;
            }
        }

        if let Some(device) = device {
            for spec in &mut device.device {
                add_device(
                    Context {
                        core,
                        meta: &meta,
                        dependency_handles: &mut dependency_handles,
                    },
                    spec,
                )
                .await?;
            }
        }

        if let Some(shadow) = shadow {
            for spec in &mut shadow.shadow {
                add_shadow(
                    Context {
                        core,
                        meta: &meta,
                        dependency_handles: &mut dependency_handles,
                    },
                    spec,
                )
                .await?;
            }
        }

        Ok(Arc::new(Self {
            meta,
            spec,
            _dependency_handles: dependency_handles,
        }))
    }

    fn status(&self) -> StackStatus {
        StackStatus {}
    }

    pub fn to_proto(&self) -> StackEntity {
        StackEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec.clone()),
            status: Some(self.status()),
        }
    }
}

impl Entity for Stack {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for Stack {
    type Package = crate::StackService;
    const KIND: &'static str = "Stack";
}

struct DependencyHandles(Vec<DependencyHandle>);

impl DependencyHandles {
    const fn new() -> Self {
        Self(Vec::new())
    }

    fn add(&mut self, handle: DependencyHandle) {
        self.0.push(handle);
    }
}

struct Context<'a> {
    core: &'a Core,
    meta: &'a EntityMeta,
    dependency_handles: &'a mut DependencyHandles,
}

impl Context<'_> {
    fn add_dependency(&mut self, meta: &EntityMeta) {
        let handle = self.meta.add_dependency(meta);
        self.dependency_handles.add(handle);
    }
}

async fn add_spi_device(
    mut ctx: Context<'_>,
    spec: &mut dipstick_proto::spi::v1::CreateDeviceRequest,
) -> Result<()> {
    let device = {
        let dipstick_proto::spi::v1::CreateDeviceRequest { meta, spec } = spec.clone();
        let spi = ctx.core.get_package::<dipstick_spi::Spi>()?;
        spi.create_device_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
            .await?
    };
    ctx.add_dependency(device.entity_meta());
    spec.spec = Some(device.spec());
    Ok(())
}

async fn add_gpio_chip(
    mut ctx: Context<'_>,
    spec: &mut dipstick_proto::gpio::v1::CreateChipRequest,
) -> Result<()> {
    let chip = {
        let dipstick_proto::gpio::v1::CreateChipRequest { meta, spec } = spec.clone();
        let gpio = ctx.core.get_package::<dipstick_gpio::Gpio>()?;
        gpio.create_chip_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
            .await?
    };
    ctx.add_dependency(chip.entity_meta());
    spec.spec = Some(chip.spec());
    Ok(())
}

async fn add_can_bus(
    mut ctx: Context<'_>,
    spec: &mut dipstick_proto::can::v1::CreateBusRequest,
) -> Result<()> {
    let bus = {
        let dipstick_proto::can::v1::CreateBusRequest { meta, spec } = spec.clone();
        let can = ctx.core.get_package::<dipstick_can::Can>()?;
        can.create_bus_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
            .await?
    };
    ctx.add_dependency(bus.entity_meta());
    spec.spec = Some(bus.spec());
    Ok(())
}

async fn add_a2l(
    mut ctx: Context<'_>,
    spec: &mut dipstick_proto::xcp::v1::CreateA2lRequest,
) -> Result<()> {
    let a2l = {
        let dipstick_proto::xcp::v1::CreateA2lRequest { meta, spec } = spec.clone();
        let xcp = ctx.core.get_package::<dipstick_xcp::XcpService>()?;
        xcp.create_a2l_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
            .await?
    };
    ctx.add_dependency(a2l.entity_meta());
    spec.spec = Some(a2l.spec());
    Ok(())
}

async fn add_xcp_session(
    mut ctx: Context<'_>,
    spec: &mut dipstick_proto::xcp::v1::CreateSessionRequest,
) -> Result<()> {
    let session = {
        let dipstick_proto::xcp::v1::CreateSessionRequest { meta, spec } = spec.clone();
        let xcp = ctx.core.get_package::<dipstick_xcp::XcpService>()?;
        xcp.create_session_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
            .await?
    };
    ctx.add_dependency(session.entity_meta());
    spec.spec = Some(session.spec());
    Ok(())
}

async fn add_device(
    mut ctx: Context<'_>,
    spec: &mut dipstick_proto::device::v1::CreateDeviceRequest,
) -> Result<()> {
    let device = {
        let dipstick_proto::device::v1::CreateDeviceRequest { meta, spec } = spec.clone();
        let device = ctx.core.get_package::<dipstick_device::DeviceService>()?;
        device
            .create_device_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
            .await?
    };
    ctx.add_dependency(device.entity_meta());
    spec.spec = Some(device.spec());
    Ok(())
}

async fn add_shadow(
    mut ctx: Context<'_>,
    spec: &mut dipstick_proto::shadow::v1::CreateShadowRequest,
) -> Result<()> {
    let bus = {
        let dipstick_proto::shadow::v1::CreateShadowRequest { meta, spec } = spec.clone();
        let shadow = ctx.core.get_package::<dipstick_shadow::ShadowService>()?;
        shadow
            .create_shadow_impl(meta.unwrap_or_default(), spec.unwrap_or_default())
            .await?
    };
    ctx.add_dependency(bus.entity_meta());
    spec.spec = Some(bus.spec());
    Ok(())
}
