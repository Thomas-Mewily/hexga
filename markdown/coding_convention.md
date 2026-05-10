
modules that: 
-  contains `utils` contains stuff that can moved in other module more frequently than other. Use the prelude to have a more stable path when needed. The same apply for `lib` that contains `utils`
-  named `prelude` contains frequently used Type, Traits, Proc Macro. They always re export their `traits` module conter part if it exist.
-  named `traits` contains only frequently Traits and Proc Macro. hey are here to not add to much type in the context, while also providing an easy way to use trait and extension trait without importing them. 
(@TODO still in progress, I'm currently splitting all the `prelude` mod with a `traits` mod when needed)