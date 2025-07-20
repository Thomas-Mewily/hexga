#![allow(unused_imports)]

/*
use hexga_engine::{prelude::*, AppRunParam, window};
use hexga_core::prelude::*;

struct App;

fn debug_millis() -> u32
{
    10
    //std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as u32 % 1000
}

impl AppLoop for App
{
    fn update(&mut self, ctx: &mut AppCtx)
    {
        println!("updated: {}", debug_millis());
    }

    fn draw(&mut self, ctx: &mut AppCtx)
    {
        println!("draw: {}", debug_millis());
        //ctx.draw();
    }
}

fn main()
{
    App.run_with_param(
        AppRunParam::game().with_default_window(
            Some
            (
                WindowParam::___().with_title("Hello world")
                .with_buttons(window::WindowButton::Maximize | window::WindowButton::Close)
                //.with_cursor_grab(window::CursorGrab::Confined)
                .with_cursor_icon(window::CursorIcon::NotAllowed)
            )
        )).unwrap();
}

/*
Todo : pouvoir mettre à jour les param sur une window déjà crée
log/warnings à setup
WASM
*/
*/

use hexga_engine::context::App;

struct MyApp;

impl App for MyApp
{

}

fn main()
{
    println!("hello world")
}