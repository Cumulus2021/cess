use anyhow::anyhow;
use ces_allocator::StatSizeAllocator;
use ces_types::AttestationProvider;
use cestory_pal::{
    AppInfo, AppVersion, ExtendMeasurement, Machine, MemoryStats, MemoryUsage, Sealing, RA,
};
use parity_scale_codec::Encode;
use std::alloc::System;
use std::io::ErrorKind;
use std::time::Duration;
use tracing::info;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub(crate) struct GraminePlatform;

impl Sealing for GraminePlatform {
    type SealError = std::io::Error;
    type UnsealError = std::io::Error;

    fn seal_data(
        &self,
        path: impl AsRef<std::path::Path>,
        data: &[u8],
    ) -> Result<(), Self::SealError> {
        std::fs::write(path, data)?;
        Ok(())
    }

    fn unseal_data(
        &self,
        path: impl AsRef<std::path::Path>,
    ) -> Result<Option<Vec<u8>>, Self::UnsealError> {
        match std::fs::read(path) {
            Err(err) if matches!(err.kind(), ErrorKind::NotFound) => Ok(None),
            other => other.map(Some),
        }
    }
}

impl RA for GraminePlatform {
    type Error = anyhow::Error;

    fn create_attestation_report(
        &self,
        provider: Option<AttestationProvider>,
        data: &[u8],
        timeout: Duration,
    ) -> Result<Vec<u8>, Self::Error> {
        match provider {
            Some(AttestationProvider::Ias) => {
                // TODO: move the key out of the binary?
                const IAS_API_KEY_STR: &str = env!("IAS_API_KEY");
                const IAS_HOST: &str = env!("IAS_HOST");
                const IAS_REPORT_ENDPOINT: &str = env!("IAS_REPORT_ENDPOINT");

                let attestation_report =
                    Some(sgx_attestation::ias::report::create_attestation_report(data, IAS_API_KEY_STR, IAS_HOST, IAS_REPORT_ENDPOINT, timeout)?);

                Ok(Encode::encode(&attestation_report))
            }
            Some(AttestationProvider::Dcap) => {
                const CESS_DCAP_PCCS_URL: &str = env!("DCAP_PCCS_URL");
                let attestation_report = Some(sgx_attestation::dcap::report::create_attestation_report(data, CESS_DCAP_PCCS_URL, timeout)?);
                Ok(Encode::encode(&attestation_report))
            }
            None => Ok(Encode::encode(&None::<AttestationProvider>)),
            _ => Err(anyhow!("Unknown attestation provider `{:?}`", provider)),
        }
    }

    fn quote_test(&self, provider: Option<AttestationProvider>) -> Result<(), Self::Error> {
        match provider {
            Some(AttestationProvider::Ias) => Ok(()),
            Some(AttestationProvider::Dcap) => Ok(()),
            None => Ok(()),
            _ => Err(anyhow!("Unknown attestation provider `{:?}`", provider)),
        }
    }

    fn mr_enclave(&self) -> Option<Vec<u8>> {
        if is_gramine() {
            sgx_api_lite::target_info()
                .map(|info| info.mr_enclave.m.to_vec())
                .ok()
        } else {
            None
        }
    }

    fn extend_measurement(&self) -> Result<ExtendMeasurement, Self::Error> {
        if is_gramine() {
            Ok(get_extend_measurement()?.expect("must in gramine enviroment"))
        } else {
            Err(anyhow!("no measurement in native mode"))
        }
    }
}

impl Machine for GraminePlatform {
    fn machine_id(&self) -> Vec<u8> {
        vec![]
    }

    fn cpu_core_num(&self) -> u32 {
        num_cpus::get() as _
    }

    #[cfg(target_arch = "x86_64")]
    fn cpu_feature_level(&self) -> u32 {
        let mut cpu_feature_level: u32 = 1;
        if is_x86_feature_detected!("avx2") {
            info!("CPU Support AVX2");
            cpu_feature_level += 1;

            if is_x86_feature_detected!("avx512f") {
                info!("CPU Support AVX512");
                cpu_feature_level += 1;
            }
        }
        cpu_feature_level
    }

    #[cfg(not(target_arch = "x86_64"))]
    fn cpu_feature_level(&self) -> u32 {
        1
    }
}

#[global_allocator]
static ALLOCATOR: StatSizeAllocator<System> = StatSizeAllocator::new(System);

impl MemoryStats for GraminePlatform {
    fn memory_usage(&self) -> MemoryUsage {
        let stats = ALLOCATOR.stats();
        MemoryUsage {
            total_peak_used: (vm_peak().unwrap_or_default() * 1024) as _,
            rust_used: stats.current as _,
            rust_peak_used: stats.peak as _,
            free: (mem_free().unwrap_or_default() * 1024) as _,
            rust_spike: stats.spike as _,
        }
    }
}

impl AppInfo for GraminePlatform {
    fn app_version() -> AppVersion {
        AppVersion {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
        }
    }
}

fn vm_peak() -> Option<usize> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    for line in status.lines() {
        if line.starts_with("VmPeak:") {
            let peak = line.split_ascii_whitespace().nth(1)?;
            return peak.parse().ok();
        }
    }
    None
}

fn mem_free() -> Option<usize> {
    let status = std::fs::read_to_string("/proc/meminfo").ok()?;
    for line in status.lines() {
        if line.starts_with("MemFree:") {
            let peak = line.split_ascii_whitespace().nth(1)?;
            return peak.parse().ok();
        }
    }
    None
}

pub(crate) fn is_gramine() -> bool {
    lazy_static::lazy_static! {
        static ref IS_GRAMINE: bool =
            std::path::Path::new("/dev/attestation/user_report_data").exists();
    }
    *IS_GRAMINE
}

pub(crate) fn print_target_info() {
    use hex_fmt::HexFmt;
    if is_gramine() {
        println!("Running in Gramine-SGX");
        let em = get_extend_measurement()
            .expect("Failed to get extend measurement")
            .expect("must in gramine enviroment");
        println!("mr_enclave       : 0x{}", HexFmt(&em.mr_enclave));
        println!("mr_signer        : 0x{}", HexFmt(&em.mr_signer));
        println!("isv_svn          : 0x{:?}", HexFmt(em.isv_svn));
        println!("isv_prod_id      : 0x{:?}", HexFmt(em.isv_prod_id));
        println!("measurement      : 0x{:?}", HexFmt(em.measurement()));
        println!("measurement hash : {:?}", em.measurement_hash());
    } else {
        println!("Running in Native mode");
    }
    println!("git revision: {}", env!("VERGEN_GIT_SHA"));
}

pub(crate) fn get_extend_measurement() -> anyhow::Result<Option<ExtendMeasurement>> {
    if !is_gramine() {
        return Ok(None);
    }
    let target_info =
        sgx_api_lite::target_info().map_err(|e| anyhow!("Failed to get target info: {:?}", e))?;
    let report_body = sgx_api_lite::report(&target_info, &[0; 64])
        .map_err(|e| anyhow!("Failed to get sgx report: {:?}", e))?
        .body;
    Ok(Some(ExtendMeasurement {
        mr_enclave: report_body.mr_enclave.m,
        mr_signer: report_body.mr_signer.m,
        isv_prod_id: report_body.isv_prod_id.to_ne_bytes(),
        isv_svn: report_body.isv_svn.to_ne_bytes(),
    }))
}
