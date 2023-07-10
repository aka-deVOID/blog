type ResponseResult<T> = std::result::Result<T, ResponseErr>;

#[derive(Debug)]
enum ResponseErr {}
