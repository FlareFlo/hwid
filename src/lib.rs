use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use deterministic_hash::DeterministicHasher;
use sysinfo::{DiskExt, ProcessorExt, RefreshKind, System, SystemExt};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct HardwareId {
	pub(crate) id: u64,
	u_str: String,
}

impl HardwareId {
	pub fn new() -> Self {
		let mut gen_string = "".to_owned();

		let kind = RefreshKind::new()
			.with_cpu()
			.with_memory()
			.with_disks_list();

		let mut system = System::new_with_specifics(kind);
		system.refresh_system();

		for processor in system.processors() {
			gen_string.push_str(processor.name());
			gen_string.push_str(processor.vendor_id());
			gen_string.push_str(processor.brand());
		}

		gen_string.push_str(&system.total_memory().to_string());

		let mut hasher = DeterministicHasher::new(DefaultHasher::new());
		gen_string.hash(&mut hasher);

		Self {
			id: hasher.finish(),
			u_str: gen_string,
		}
	}
}

#[cfg(test)]
mod test {
	use crate::HardwareId;

	#[test]
	fn test() {
		assert_eq!(HardwareId::new(), HardwareId::new())
	}
}