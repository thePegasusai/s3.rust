// Durability: A production-ready S3 storage system would need to ensure that data is stored durably and can be retrieved even in the event of failures. This could be achieved by implementing data replication, backups, and versioning.

    struct Object {
        key: String,
        data: Vec<u8>,
        metadata: HashMap<String, String>,
        version: u32,
    }

    struct Bucket {
        objects: HashMap<String, Vec<Object>>,
    }

    impl Bucket {
        fn create_object(&mut self, object: Object) {
            let objects = self.objects.entry(object.key.clone()).or_insert(vec![]);
            objects.push(object);
        }

        fn retrieve_object(&self, key: &str) -> Option<&Object> {
            self.objects.get(key)?.last()
        }

        fn retrieve_object_version(&self, key: &str, version: u32) -> Option<&Object> {
            self.objects.get(key)?.iter().find(|object| object.version == version)
        }
    }
