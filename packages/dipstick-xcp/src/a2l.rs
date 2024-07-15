use std::sync::{Arc, Mutex};

use dipstick_core::{Core, Entity, EntityKind, EntityMeta};
use dipstick_proto::xcp::v1::{
    A2lDataType, A2lEntity, A2lFile, A2lMeasurement, A2lModule, A2lProject, A2lSpec, A2lStatus,
};
use tonic::{Result, Status};

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
        .map(map_file)
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

    pub fn get_measurement(&self, measurement_name: &str) -> Option<A2lMeasurement> {
        let storage = self.storage.lock().unwrap();
        for project in &storage.a2l_file.project {
            for module in &project.module {
                for meas in &module.measurement {
                    if meas.name == measurement_name {
                        return Some(meas.clone());
                    }
                }
            }
        }
        None
    }
}

impl Entity for A2l {
    fn entity_meta(&self) -> &EntityMeta {
        &self.meta
    }
}

impl EntityKind for A2l {
    const PACKAGE: &'static str = crate::PACKAGE;
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

fn map_file(a2l: a2lfile::A2lFile) -> A2lFile {
    let module = a2l.project.module.into_iter().map(map_module).collect();
    let project = A2lProject { module };
    A2lFile {
        project: Some(project),
    }
}

fn map_module(module: a2lfile::Module) -> A2lModule {
    let measurement = module
        .measurement
        .into_iter()
        .map(map_measurement)
        .collect();
    A2lModule { measurement }
}

fn map_measurement(meas: a2lfile::Measurement) -> A2lMeasurement {
    let datatype = match meas.datatype {
        a2lfile::DataType::Ubyte => A2lDataType::Ubyte,
        a2lfile::DataType::Sbyte => A2lDataType::Sbyte,
        a2lfile::DataType::Uword => A2lDataType::Uword,
        a2lfile::DataType::Sword => A2lDataType::Sword,
        a2lfile::DataType::Ulong => A2lDataType::Ulong,
        a2lfile::DataType::Slong => A2lDataType::Slong,
        a2lfile::DataType::AUint64 => A2lDataType::AUint64,
        a2lfile::DataType::AInt64 => A2lDataType::AInt64,
        a2lfile::DataType::Float16Ieee => A2lDataType::F16Ieee,
        a2lfile::DataType::Float32Ieee => A2lDataType::F32Ieee,
        a2lfile::DataType::Float64Ieee => A2lDataType::F64Ieee,
    };
    A2lMeasurement {
        name: meas.name,
        long_identifier: meas.long_identifier,
        datatype: datatype as _,
        ecu_address: meas.ecu_address.map(|addr| addr.address),
        ecu_address_extension: meas
            .ecu_address_extension
            .map(|ext| i32::from(ext.extension)),
    }
}
