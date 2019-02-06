use crate::config::Config;
use crate::operations;
use crate::operations::transformations::apply_operations_on_image;
use crate::operations::Operation;
use crate::processor::ProcessMutWithConfig;

pub(crate) struct ImageOperationsProcessor<'a> {
    buffer: &'a mut image::DynamicImage,
}

impl<'a> ImageOperationsProcessor<'a> {
    pub fn new(buffer: &'a mut image::DynamicImage) -> ImageOperationsProcessor {
        ImageOperationsProcessor { buffer }
    }

    fn apply_operations(&mut self, ops: &[Operation]) -> Result<(), String> {
        println!("Applying image operations.");

        apply_operations_on_image(&mut self.buffer, ops)
    }
}

impl<'a> ProcessMutWithConfig<Result<(), String>> for ImageOperationsProcessor<'a> {
    fn process_mut(&mut self, _config: &Config) -> Result<(), String> {
        // If we don't have the script option defined, do nothing.

        dbg!("TODO: ensure operations are handled per use case");
        if dbg!(false) {
            let operations: Result<Vec<Operation>, String> = Ok(vec![]);

            self.apply_operations(&operations?)
        } else {
            Ok(())
        }
    }
}
