mod enumerated;
mod record;
use record::Record;

#[allow(dead_code)]
struct EnumMap<K, V> {
    values: [Record<K, V>],
}
