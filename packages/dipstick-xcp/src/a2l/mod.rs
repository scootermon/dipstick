use std::sync::{Arc, Mutex};

use dipstick_core::{Core, Entity, EntityKind, EntityMeta};
use dipstick_proto::xcp::v1::{
    A2lByteOrder, A2lEntity, A2lFile, A2lFullCharacteristic, A2lMeasurement, A2lSpec, A2lStatus,
};
use tonic::{Result, Status};

pub mod codec;
mod mapping;

pub struct A2l {
    meta: EntityMeta,
    spec: Mutex<A2lSpec>,
    storage: Mutex<Storage>,
}

impl A2l {
    pub async fn new(core: &Core, meta: EntityMeta, spec: A2lSpec) -> Result<Arc<Self>> {
        let this = Arc::new(Self {
            meta,
            spec: Mutex::new(A2lSpec::default()),
            storage: Mutex::new(Storage::new()),
        });
        Arc::clone(&this).update_spec(core, spec).await?;
        Ok(this)
    }

    async fn update_spec(&self, core: &Core, new_spec: A2lSpec) -> Result<()> {
        let file = new_spec.file.clone().unwrap_or_default();
        let content = core.read_file_to_string(&file).await?;
        let mut log_msgs = Vec::new();
        let a2l_file = a2lfile::load_from_string(
            &content,
            new_spec.a2ml_spec.clone(),
            &mut log_msgs,
            new_spec.strict_parsing(),
        )
        .map(mapping::map_file)
        .map_err(|err| Status::invalid_argument(format!("{err}")))?;

        {
            let mut spec = self.spec.lock().unwrap();
            let mut storage = self.storage.lock().unwrap();
            *spec = new_spec;
            storage.a2l_file = a2l_file;
            storage.log_msgs = log_msgs;
        }
        Ok(())
    }

    pub fn spec(&self) -> A2lSpec {
        self.spec.lock().unwrap().clone()
    }

    pub fn status(&self) -> A2lStatus {
        self.storage.lock().unwrap().to_proto()
    }

    pub fn to_proto(&self) -> A2lEntity {
        A2lEntity {
            meta: Some(self.meta.to_proto()),
            spec: Some(self.spec()),
            status: Some(self.status()),
        }
    }

    pub fn get_measurement(&self, measurement_name: &str) -> Result<A2lMeasurement> {
        let storage = self.storage.lock().unwrap();
        let modules = storage
            .a2l_file
            .project
            .iter()
            .flat_map(|project| project.module.iter());
        for module in modules {
            let Some(measurement) = module
                .measurement
                .iter()
                .find(|measurement| measurement.name == measurement_name)
            else {
                continue;
            };

            let mut measurement = measurement.clone();
            if measurement.byte_order() == A2lByteOrder::Unspecified {
                measurement.set_byte_order(module.byte_order());
            }
            return Ok(measurement);
        }
        Err(Status::not_found(format!(
            "measurement {measurement_name:?} not found"
        )))
    }

    pub fn get_characteristic(&self, characteristic_name: &str) -> Result<A2lFullCharacteristic> {
        let storage = self.storage.lock().unwrap();
        let modules = storage
            .a2l_file
            .project
            .iter()
            .flat_map(|project| project.module.iter());
        for module in modules {
            let Some(characteristic) = module
                .characteristic
                .iter()
                .find(|characteristic| characteristic.name == characteristic_name)
            else {
                continue;
            };
            let record_layout = module
                .record_layout
                .iter()
                .find(|layout| layout.name == characteristic.deposit)
                .ok_or_else(|| {
                    Status::not_found("record layout referenced by characteristic not found")
                })?;
            let mut characteristic = characteristic.clone();
            if characteristic.byte_order() == A2lByteOrder::Unspecified {
                characteristic.set_byte_order(module.byte_order());
            }
            return Ok(A2lFullCharacteristic {
                characteristic: Some(characteristic),
                record_layout: Some(record_layout.clone()),
            });
        }
        Err(Status::not_found("characteristic not found"))
    }
}

impl Entity for A2l {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for A2l {
    type Package = crate::XcpService;
    const KIND: &'static str = "A2l";
}

struct Storage {
    a2l_file: A2lFile,
    log_msgs: Vec<a2lfile::A2lError>,
}

impl Storage {
    fn new() -> Self {
        Self {
            a2l_file: A2lFile::default(),
            log_msgs: Vec::new(),
        }
    }

    fn to_proto(&self) -> A2lStatus {
        A2lStatus {
            file: Some(self.a2l_file.clone()),
            warnings: self.log_msgs.iter().map(|err| err.to_string()).collect(),
        }
    }
}
