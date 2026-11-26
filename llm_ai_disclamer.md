Hello there!

My default/main editor is Codium, and it is free from any AI / LLM / Agent in my workflow.
It took me sometimes multiple attempts and full rewrites from scratch with different approaches to create this code.

The vast majority of the code in this repo was:
- written by hand most of the time.
- taken and adapted from existing codebases (e.g., `hexga_event_loop` re-exports `winit`, `hexga_wgpu` re-exports `wgpu`, `hexga_bit` re-exports `bytemuck`...).
- piece of code written by any llm are super rare, super short (a fews line usually) and always proofread by me. 

The reason why I'm still writing most of the code manually is because I care about:
- Code quality, 
- Re-usability
- Long Term Maintainabilty (even if most of the hexga crates are experimental right now).

There are some stuff I'm proud of like 
- having a generic array-size fixed vector with `hexga_math::prelude::Vector<T, const N: usize>` that also has fields `x`, `y`, `z`, and `w` depending on the dimension using some safe `Deref`/`DerefMut` hack, which is something that other crates like `glam` don't have, they have unrolled code for every kind of vector (float, int, bool...).

Some minority of the code was written by LLM, such as: 
- proc macro logic (which is hard to write by hand)
- some doc comments / examples
- some tests code

I'm to lazy to write a conclusion, and I will not do the supplice to generate a dummy one by a LLM, so thank for reading.

- Mewily.

TL;DR: Mostly handwritten with selective AI help for boilerplate and complex macros. *<- This line was LLM written then human edited >:)*


Update 26/11/2026:

I started using AI a bit. Recently I had some massive but dummy/straightforward refactoring to do (mainly after modifying the collection trait, which introduced over 900 errors in multiple crates). I tried to refactor it manually for an hour and a half, got bored because it was trivial, and asked AI to do it.

I still review every bit of code it produces and edit it again as many time as needed, because I care about architecture, how different pieces of code fit/glue and interact together, and how the code will be used by user.