# easy-async-cl3

A high-level, async-first Rust wrapper for OpenCL with intelligent multi-device management and declarative task execution.

## Overview

`easy-async-cl3` provides a modern, ergonomic interface to OpenCL that embraces Rust's async/await paradigm. The library automatically manages resources, distributes work across multiple devices, and provides compile-time safety guarantees.

### Key Features

- **Async/Await Integration**: All GPU operations return futures for seamless async workflows
- **Automatic Multi-Device Support**: Intelligent work distribution across multiple GPUs based on device capabilities
- **Type-Safe API**: Compile-time guarantees prevent common errors (e.g., using unbuilt programs)
- **Declarative Task Building**: Fluent builder pattern for constructing GPU tasks
- **Zero-Cost Abstractions**: RAII-based resource management with no runtime overhead
- **Comprehensive OpenCL Support**: Full support for OpenCL 1.1 through 3.0 features including Pipes, SVM, and Images
- **Built-in Profiling**: Optional performance measurement with negligible overhead

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
easy-async-cl3 = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quick Start

```rust
use easy_async_cl3::{
    async_executor::AsyncExecutor,
    cl_types::memory_flags::MemoryFlags,
    error::ClError,
};

#[tokio::main]
async fn main() -> Result<(), ClError> {
    // Initialize executor with best available platform
    let executor = AsyncExecutor::new_best_platform()?;
    
    // Define and build kernel
    let source = r#"
        kernel void vector_add(global float* a, global const float* b) {
            size_t i = get_global_id(0);
            a[i] += b[i];
        }
    "#;
    let program = executor.build_program(source.to_string(), None)?;
    let kernel = executor.create_kernel(&program, "vector_add")?;
    
    // Prepare data
    let size = 1_000_000;
    let mut a = vec![1.0f32; size];
    let b = vec![2.0f32; size];
    
    // Create GPU buffers
    let buf_a = executor.create_buffer(
        &[MemoryFlags::ReadWrite, MemoryFlags::CopyHostPtr],
        size * std::mem::size_of::<f32>(),
        a.as_mut_ptr() as *mut _
    )?;
    let buf_b = executor.create_buffer(
        &[MemoryFlags::ReadOnly, MemoryFlags::CopyHostPtr],
        size * std::mem::size_of::<f32>(),
        b.as_ptr() as *mut _
    )?;
    
    // Execute task
    executor.create_task(kernel)
        .arg_buffer(0, &buf_a)
        .arg_buffer(1, &buf_b)
        .global_work_dims(size, 1, 1)
        .read_buffer(&buf_a, &mut a)
        .run()
        .await?;
    
    assert_eq!(a[0], 3.0);
    Ok(())
}
```

## Advanced Features

### Multi-Device Execution

The library automatically detects and utilizes all available compute devices:

```rust
let executor = AsyncExecutor::new_best_platform_with_options(true)?; // Enable profiling

let report = executor.create_task(kernel)
    .arg_buffer(0, &buffer)
    .global_work_dims(10_000_000, 1, 1)
    .run()
    .await?;

println!("Execution time: {} μs", report.total_kernel_duration_ns() / 1000);
```

### Shared Virtual Memory (OpenCL 2.0+)

Zero-copy memory sharing between CPU and GPU:

```rust
let mut svm_buffer = executor.create_svm_buffer::<f32>(
    &[MemoryFlags::ReadWrite], 
    1024
)?;

executor.create_task(kernel)
    .arg_svm(0, &svm_buffer)
    .global_work_dims(1024, 1, 1)
    .run()
    .await?;

// Direct CPU access without explicit copy
let queue = &executor.get_queues()[0];
let mapped = svm_buffer.map_mut(queue, &vec![MemoryFlags::ReadWrite])?;
println!("Result: {}", mapped[0]);
```

### Image Processing

Native support for OpenCL images with hardware-accelerated filtering:

```rust
use easy_async_cl3::cl_types::cl_image::{
    ClImageFormats, ClImageDesc, image_type::ClImageType
};

let format = ClImageFormats::rgba_unorm_int8();
let desc = ClImageDesc {
    image_type: ClImageType::Image2D,
    image_width: Some(1920),
    image_height: Some(1080),
    ..Default::default()
};

let image = executor.create_image(
    &[MemoryFlags::ReadWrite],
    &format,
    &desc,
    std::ptr::null_mut()
)?;

executor.create_task(kernel)
    .arg_image(0, &image)
    .global_work_dims(1920, 1080, 1)
    .run()
    .await?;
```

### Pipes for Inter-Kernel Communication (OpenCL 2.0+)

Stream data between kernels without CPU involvement:

```rust
use easy_async_cl3::cl_types::cl_pipe::ClPipe;

let pipe = ClPipe::new(
    executor.get_context().as_ref(),
    &[MemoryFlags::ReadWrite],
    4,    // packet size (bytes)
    1024  // max packets
)?;

// Producer writes to pipe
executor.create_task(producer_kernel)
    .arg_pipe(0, &pipe)
    .global_work_dims(1024, 1, 1)
    .run()
    .await?;

// Consumer reads from pipe
executor.create_task(consumer_kernel)
    .arg_pipe(0, &pipe)
    .global_work_dims(1024, 1, 1)
    .run()
    .await?;
```

## Architecture

The library is structured in three main layers:

1. **AsyncExecutor**: High-level interface managing platforms, devices, and command queues
2. **TaskBuilder**: Declarative API for constructing and executing GPU tasks
3. **CL Types**: Type-safe wrappers around OpenCL objects (buffers, images, kernels, etc.)

Work is automatically distributed across available devices based on their compute capabilities and memory capacity.

## Documentation

- [API Documentation](https://docs.rs/easy-async-cl3) - Complete API reference
- [Examples](./tests/api_guide.rs) - Comprehensive usage examples
- [OpenCL Specification](https://www.khronos.org/opencl/) - Kernel programming reference

## Requirements

- Rust 1.70 or later
- OpenCL runtime (provided by GPU vendor drivers)
- Tokio async runtime

## Use Cases

- High-performance scientific computing
- Real-time image and video processing
- Machine learning inference and training
- Cryptographic operations
- Financial modeling and simulations
- Parallel data analytics

## Contributing

Contributions are welcome. Please ensure all tests pass and follow the existing code style.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

Built on the [cl3](https://github.com/kenba/cl3) library.
