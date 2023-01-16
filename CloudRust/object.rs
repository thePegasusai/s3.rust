# \\Create a struct that represents an object: This struct should contain the object's key, data, and metadata.

struct Object {
    key: String,
    data: Vec<u8>,
    metadata: HashMap<String, String>,
}

# \\Implement a struct that represents a bucket: This struct should contain a vector of objects, and methods for creating, deleting and listing objects.

struct Bucket {
    objects: Vec<Object>,

    fn create_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    fn delete_object(&mut self, object_key: &str) {
        self.objects.retain(|object| object.key != object_key);
    }

    fn list_objects(&self) -> Vec<&Object> {
        self.objects.iter().collect()
    }
}

#//Implement a struct that represents a storage service: This struct should contain a vector of buckets, and methods for creating, deleting and listing buckets./
struct StorageService {
    buckets: Vec<Bucket>,

    fn create_bucket(&mut self, bucket_name: &str) {
        self.buckets.push(Bucket { objects: vec![] });
    }

    fn delete_bucket(&mut self, bucket_name: &str) {
        self.buckets.retain(|bucket| bucket.name != bucket_name);
    }

    fn list_buckets(&self) -> Vec<&Bucket> {
        self.buckets.iter().collect()
    }
}
