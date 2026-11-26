use hexga_math::prelude::*;

fn main() {
    let mut pos = Vec2::ZERO;

    let enemy0 = vec2(67., 42.);
    let enemy1 = vec2(69., 42.);

    let target = enemy0.lerp(enemy1, 0.5); // lerp is good a non time dependend computation

    '_gameloop: for _ in 0..10 {
        let dt: float = 1. / 60.;

        // Even if dt change, the integration remain constant
        pos = Integrate(pos, dt).lerp(target, 0.5);
        // or
        pos.lerp_dt(target, 0.5, dt);
    }
}
