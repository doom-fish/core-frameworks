#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SMPTETimeType {
    Type2398,
    Type24,
    Type25,
    Type2997,
    Type2997drop,
    Type30,
    Type30drop,
    Type50,
    Type5994,
    Type5994drop,
    Type60,
    Type60drop,
}
