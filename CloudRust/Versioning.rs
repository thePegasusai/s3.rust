\\ Versioning: A production-ready S3 storage system would need to have versioning feature enabled, which allows users to store multiple versions of an object in a bucket.

    struct Object {
        key: String,
        data: Vec<u8>,
        metadata: HashMap<String, String>,
        version_id: String,
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

        fn retrieve_object_version(&self, key: &str, version_id: &str) -> Option<&Object> {
            self.objects.get(key)?.iter().find(|object| object.version_id == version_id)
        }
    }
