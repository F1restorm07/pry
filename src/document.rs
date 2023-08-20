use std::sync::{ Arc, Mutex, RwLock };
use std::hash::Hasher;

// use roaring::RoaringBitmap; maybe do something with this in the future
use once_cell::sync::Lazy;
use hashbrown::{ HashSet, HashMap };
use memmap2:: Mmap;
use crate::COLLECTION_POOL_PATH;
use twox_hash::XxHash64;

type Fst = fst::raw::Fst<Vec<u8>>;

static DOCUMENT_COLLECTION: Lazy<Arc<RwLock<HashMap<IndexKey, Arc<Document>>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(HashMap::new()))
});
static ACQUIRE_LOCK: Lazy<Arc<Mutex<()>>> = Lazy::new(|| {
    Arc::new(Mutex::new(()))
});
static CONSOLIDATE_LOCK: Lazy<Arc<Mutex<()>>> = Lazy::new(|| {
    Arc::new(Mutex::new(()))
});
static ACCESS_LOCK: Lazy<Arc<Mutex<()>>> = Lazy::new(|| {
    Arc::new(Mutex::new(()))
});
static CONSOLIDATE_DOC_SET: Lazy<Arc<RwLock<HashSet<IndexKey>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(HashSet::new()))
});

pub struct IndexPool;
pub struct IndexBuilder;

#[derive(Default, Debug)]
pub struct IndexPending {
    pop: Arc<RwLock<HashSet<u8>>>,
    push: Arc<RwLock<HashSet<u8>>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct IndexKey {
    collection_id: u32,
    bucket_id: u32,
}

// #[derive(Debug)]
pub struct Document {
    pub fst: Fst,
    pub target: IndexKey,
    pending: IndexPending,
}

impl Document {
    pub fn consolidate_next(&self) {
        if !CONSOLIDATE_DOC_SET.read().unwrap().contains(&self.target) {
            CONSOLIDATE_DOC_SET.write().unwrap().insert(self.target);


        }
    }
}

impl IndexKey {
    fn from_str(collection: &str, bucket: &str) -> Self {
        let mut collection_hasher = XxHash64::with_seed(6);
        let mut bucket_hasher = XxHash64::with_seed(6);

        collection_hasher.write(collection.as_bytes());
        bucket_hasher.write(bucket.as_bytes());

        Self {
            collection_id: collection_hasher.finish() as u32,
            bucket_id: bucket_hasher.finish() as u32,
        } 
    }
}

impl IndexPool {
    pub fn acquire_open(
        collection_str: &str,
        document_key: IndexKey,
        collection: &Arc<RwLock<HashMap<IndexKey, Arc<Document>>>>,
        ) -> Result<Arc<Document>, Box<dyn std::error::Error>> {
        println!("IndexPool::acquire_open");
        match IndexBuilder::build(document_key) {
            Ok(doc) => {
                let mut collection_write = collection.write().unwrap();
                let doc_box = Arc::new(doc);

                collection_write.insert(document_key, doc_box.clone());

                // println!("collection_pool document {:?}", collection_write.values().last().unwrap().target);

                Ok(doc_box)
            }
            Err(e) => Err(Box::new(e))
        }
    }
    pub fn acquire_cache(
        collection_str: &str,
        document_key: IndexKey,
        collection: &Arc<Document>,
        ) -> Result<Arc<Document>, Box<dyn std::error::Error>> {
        println!("IndexPool::acquire_cache");
        Ok(collection.clone())
    }

}

impl IndexPool {
    pub fn acuqire(collection: &str, document: &str) -> Result<Arc<Document>, Box<dyn std::error::Error>> {
        println!("IndexPool::acquire");
        let document_key = IndexKey::from_str(collection, document);
        let _acquire_lock = ACQUIRE_LOCK.lock().unwrap();
        let collection_read = DOCUMENT_COLLECTION.read().unwrap();

        println!("document key {document_key:?}\n {:?}", collection_read.keys());

        if let Some(document_fst) = collection_read.get(&document_key) {
            Self::acquire_cache(collection, document_key, document_fst)
        } else {
            drop(collection_read);
            Self::acquire_open(collection, document_key, &*DOCUMENT_COLLECTION)
        }

    }

    // populate fsts
    pub fn consolidate(force: bool) {
        println!("IndexPool::consolidate");

        let _consolidate_lock = CONSOLIDATE_LOCK.lock().unwrap();

        if CONSOLIDATE_DOC_SET.read().unwrap().is_empty() {
            println!("no documents to consolidate");
            return;
        }

        let consolidate_keys: Vec<IndexKey> = Vec::new();

        {
            let _access = ACCESS_LOCK.lock().unwrap();
        }

        todo!();
    }
}

impl IndexBuilder {
    pub fn build(collection_key: IndexKey) -> Result<Document, fst::Error> {
        println!("IndexBuilder::build");
        Self::open(collection_key.collection_id, collection_key.bucket_id)
            .map(|fst| {
                Document {
                    fst,
                    target: collection_key,
                    pending: IndexPending::default(),
                }
            })
    }
    pub fn open(collection_id: u32, bucket_id: u32) -> Result<Fst, fst::Error> {
        println!("IndexBuilder::open");
        let bucket_path = Self::path(collection_id, Some(bucket_id));

        if bucket_path.exists() {
            println!("opening existing fst file");
            let mem_map = unsafe { Mmap::map(&std::fs::File::open(bucket_path)?)? };
            Fst::new(mem_map.as_ref().to_vec())
        } else {
            // fst contained in memory, needs to be consolidated in order to create fst file
            // let mut fst_file = std::io::BufWriter::new(std::fs::File::create(bucket_path)?);

            println!("opening new fst into memory");
            Ok(fst::raw::Builder::memory().into_fst())
        }
    }
    pub fn path(collection_id: u32, bucket_id: Option<u32>) -> std::path::PathBuf {
        println!("IndexBuilder::path");

        let mut path = std::path::PathBuf::from(COLLECTION_POOL_PATH).join(format!("{}", collection_id));

        if let Some(bucket) = bucket_id {
            return path.join(format!("{}.fst", bucket));
            // println!("{:?}", path);
        }
        println!("{path:?}");
        path
    }
}

impl IndexPending {

}

impl Document {

}

// reference sonic search engine for indexing stuff
// this is a basic implementation of sonic's storage system
//
// a series of collections house buckets within them with an overall fst pool
// collections, buckets, and overall pool should be thread-safe
