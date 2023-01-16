// Life cycle: A production-ready S3 storage system would need to have life cycle feature enabled, which allows users to automate the process of moving objects to the appropriate storage class or to delete them.

struct LifeCycleRule {
    key_prefix: String,
    status: LifeCycleStatus,
    transition_days: Option<u32>,
    expiration_days: Option<u32>,
}

enum LifeCycleStatus {
    Enabled,
    Disabled,
}

struct Bucket {
    name: String,
    objects: HashMap<String, Object>,
    life_cycle_rules: Vec<LifeCycleRule>,
}

impl Bucket {
    fn apply_life_cycle_rules(&mut self) {
        let current_time = Utc::now();
        for (key, object) in &mut self.objects {
            for rule in &self.life_cycle_rules {
                if key.starts_with(&rule.key_prefix) {
                    match rule.status {
                        LifeCycleStatus::Enabled => {
                            if let Some(transition_days) = rule.transition_days {
                                let transition_date = object.metadata["creation_date"] + Duration::days(transition_days);
                                if current_time > transition_date {
                                    object.storage_class = "STANDARD_IA";
                                }
                            }
