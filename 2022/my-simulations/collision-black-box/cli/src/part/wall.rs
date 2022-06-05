use crate::util;
use colored::Colorize;
use rand_distr::Distribution;
use rustyline;

const RADIUS: f64 = 0.3502;
const RADIUS_ERR: f64 = 0.0092;

const RESTITUTION_COEFFICIENT: f64 = 0.696;
const RESTITUTION_COEFFICIENT_ERR: f64 = 0.035;

const FRICTION_COEFFICIENT: f64 = 0.43;
const FRICTION_COEFFICIENT_ERR: f64 = 0.02;

const MOMENT_COEFFICIENT: f64 = 0.799;
const MOMENT_COEFFICIENT_ERR: f64 = 0.002;

const DISTANCE: f64 = 0.5;
const DISTANCE_ERR: f64 = 0.01;

const PROCESSING: f64 = 0.2;
const PROCESSING_ERR: f64 = 0.1;

const V_MIN: f64 = 0.5;
const V_MAX: f64 = 10.;
const V_ERR_ABS: f64 = 0.1;
const V_ERR_REL: f64 = 0.05;

const Θ_MIN: f64 = 0.;
const Θ_MAX: f64 = 75.;
const Θ_ERR: f64 = 1.;

const ANGULAR_MIN: f64 = -50.;
const ANGULAR_MAX: f64 = 50.;

const ANGULAR_ERR_ABS: f64 = 0.2;
const ANGULAR_ERR_REL: f64 = 0.05;

pub fn run() -> Result<(), rustyline::error::ReadlineError> {
    let mut rl = rustyline::Editor::<()>::new();

    let perturb_v = util::Perturber::new(V_ERR_ABS, V_ERR_REL);
    let perturb_θ = util::Perturber::new(Θ_ERR, 0.);
    let perturb_ω = util::Perturber::new(ANGULAR_ERR_ABS, ANGULAR_ERR_REL);

    let dist_r = rand_distr::Normal::new(RADIUS, RADIUS_ERR).unwrap();
    let dist_μ = rand_distr::Normal::new(FRICTION_COEFFICIENT, FRICTION_COEFFICIENT_ERR).unwrap();
    let dist_c =
        rand_distr::Normal::new(RESTITUTION_COEFFICIENT, RESTITUTION_COEFFICIENT_ERR).unwrap();
    let dist_β = rand_distr::Normal::new(MOMENT_COEFFICIENT, MOMENT_COEFFICIENT_ERR).unwrap();

    let dist_h = rand_distr::Normal::new(DISTANCE, DISTANCE_ERR).unwrap();
    let dist_processing = rand_distr::Normal::new(PROCESSING, PROCESSING_ERR).unwrap();

    let mut rng = rand::thread_rng();

    for i in 1.. {
        println!(
            "{} {} {}",
            ">>>".cyan().dimmed(),
            format!("trial #{}…", i).bright_cyan().bold(),
            format!("(press {} or {} to exit)", "ctrl+C".bold(), "ctrl+D".bold()).cyan()
        );

        let v = perturb_v
            .perturb(
                &mut rng,
                util::read_float(&mut rl, "launch speed (m/s):         ", V_MIN, V_MAX)?,
            )
            .max(V_MIN / 2.);

        let θ = perturb_θ
            .perturb(
                &mut rng,
                util::read_float(&mut rl, "launch angle (° to normal): ", Θ_MIN, Θ_MAX)?,
            )
            .clamp(0., 90.)
            .to_radians();

        let ω = perturb_ω.perturb(
            &mut rng,
            util::read_float(
                &mut rl,
                "angular speed (rad/s):      ",
                ANGULAR_MIN,
                ANGULAR_MAX,
            )?,
        );

        let r = dist_r.sample(&mut rng);
        let c = dist_c.sample(&mut rng);
        let μ = dist_μ.sample(&mut rng);
        let β = dist_β.sample(&mut rng);

        let vy = v * θ.cos();
        let vx = v * θ.sin();

        // magnitude of maximum horizontal impulse due to friction
        let δvx_max = μ * vy * (1. + c);

        // horizontal impulse if collision results in non-slip rolling:
        //   vx + δvx = vx' = ω' r,
        //   β r² ω - δvx r = β r² ω',
        //   ∴  ω r - δvx / β = vx + δvx,
        //   ∴  δvx = (ω r - vx) / (1 + 1/β).
        let δvx_nonslip = (ω * r - vx) / (1. + 1. / β);

        // actual impulse tends to non-slip impulse, but capped in magnitude by δvx_max
        let δvx = δvx_nonslip.clamp(-δvx_max, δvx_max);

        let vy_new = c * vy;
        let vx_new = vx + δvx;
        let ω_new = ω - δvx / β / r;

        let v_new = (vx_new * vx_new + vy_new * vy_new).sqrt();
        let θ_new = (vx_new / vy_new).atan().to_degrees();

        println!("{}", "  running…".green());

        // wait for ball to return
        std::thread::sleep(std::time::Duration::from_secs_f64(
            dist_h.sample(&mut rng) * (1. / vy + 1. / vy_new)
                + dist_processing.sample(&mut rng).max(0.),
        ));

        println!(
            "  rebound speed: {}\n  rebound angle: {}\n  angular speed: {}\n",
            format!("{:<+7.3} m/s", v_new).bold(),
            format!("{:<+7.3} ° to normal", θ_new).bold(),
            format!("{:<+7.3} rad/s", ω_new).bold(),
        );
    }

    Ok(())
}
