use crate::config::Config;
use crate::operations::transformations::apply_operations_on_image;
use crate::operations::Operation;
use crate::processor::ProcessMutWithConfig;

pub(crate) struct ImageOperationsProcessor<'a> {
    buffer: &'a mut image::DynamicImage,
    operation: Option<Operation>,
}

impl<'a> ImageOperationsProcessor<'a> {
    pub fn new(
        buffer: &'a mut image::DynamicImage,
        operation: Option<Operation>,
    ) -> ImageOperationsProcessor {
        ImageOperationsProcessor { buffer, operation }
    }

    fn apply_operations(&mut self, ops: &[Operation]) -> Result<(), String> {
        println!("Applying image operation.");

        apply_operations_on_image(&mut self.buffer, ops)
    }
}

impl<'a> ProcessMutWithConfig<Result<(), String>> for ImageOperationsProcessor<'a> {
    fn process_mut(&mut self, _config: &Config) -> Result<(), String> {
        if let Some(op) = &self.operation {
            self.apply_operations(&[op.clone()])
        } else {
            Ok(())
        }
    }
}
