Hello there!

The vast majority of the code in this repo was:
- written by hand
- taken and adapted from existing codebases (e.g., `hexga_event_loop` re-exports `winit`, `hexga_wgpu` re-exports `wgpu`, `hexga_bit` re-exports `bytemuck`...).

My default/main editor is Codium, and it is free from any AI / LLM / Agent in my workflow.
It took me sometimes multiple attempts and full rewrites from scratch with different approaches to create this code.

The reason why I'm still writing most of the code manually is because I care about:
- Code quality, 
- Re-usability
- Long Term Maintainabilty (even if most of the hexga crates are experimental right now).

There are some stuff I'm proud of like 
- having a generic array-size fixed vector with `hexga_math::prelude::Vector<T, const N: usize>` that also has fields `x`, `y`, `z`, and `w` depending on the dimension using some safe `Deref`/`DerefMut` hack, which is something that other crates like `glam` don't have, they have unrolled code for every kind of vector (float, int, bool...).

Some minority of the code was written by LLM, such as: 
- proc macro logic (which is hard to write)
- some doc comments / examples
- some tests code

I'm to lazy to write a conclusion, and I will not do the supplice to generate a dummy one by a LLM, so thank for reading.

- Mewily.

TL;DR: Mostly handwritten with selective AI help for boilerplate and complex macros. *<- This line was LLM written then human edited*