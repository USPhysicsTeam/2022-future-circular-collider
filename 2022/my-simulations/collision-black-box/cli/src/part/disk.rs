use crate::util;
use colored::Colorize;
use rand_distr::Distribution;
use rustyline;

const RADIUS_MYSTERY: f64 = 0.145;
const RADIUS_MYSTERY_ERR: f64 = 0.002;

const MASS_MYSTERY: f64 = 1.414;
const MASS_MYSTERY_ERR: f64 = 0.018;

const RADIUS_PROBE: f64 = 0.250;
const RADIUS_PROBE_ERR: f64 = 0.001;

const MASS_PROBE_MIN: f64 = 1.0;
const MASS_PROBE_MAX: f64 = 5.0;
const MASS_PROBE_ERR_ABS: f64 = 0.05;
const MASS_PROBE_ERR_REL: f64 = 0.01;

const COEFFICIENT_RESTITUTION: f64 = 0.852;
const COEFFICIENT_RESTITUTION_ERR: f64 = 0.022;

const Θ_MIN: f64 = -90.;
const Θ_MAX: f64 = 90.;
const Θ_ERR: f64 = 0.1;

const DISTANCE: f64 = 2.;
const DISTANCE_ERR: f64 = 0.1;

const X: f64 = 0.696;
const Y: f64 = -1.230;
const POS_ERR: f64 = 0.003;

const PROCESSING: f64 = 0.2;
const PROCESSING_ERR: f64 = 0.1;

const V_MIN: f64 = 0.5;
const V_MAX: f64 = 10.;
const V_ERR_ABS: f64 = 0.05;
const V_ERR_REL: f64 = 0.01;

const S_MIN: f64 = -2.;
const S_MAX: f64 = 2.;
const S_ERR: f64 = 0.002;

pub fn run() -> Result<(), rustyline::error::ReadlineError> {
    let mut rl = rustyline::Editor::<()>::new();

    let perturb_𝑣 = util::Perturber::new(V_ERR_ABS, V_ERR_REL);
    let perturb_𝑠 = util::Perturber::new(S_ERR, 0.);
    let perturb_𝜃 = util::Perturber::new(Θ_ERR, 0.);
    let perturb_𝑚 = util::Perturber::new(MASS_PROBE_ERR_ABS, MASS_PROBE_ERR_REL);

    let dist_𝑟_probe = rand_distr::Normal::new(RADIUS_PROBE, RADIUS_PROBE_ERR).unwrap();
    let dist_𝑟_mystery = rand_distr::Normal::new(RADIUS_MYSTERY, RADIUS_MYSTERY_ERR).unwrap();
    let dist_𝑚_mystery = rand_distr::Normal::new(MASS_MYSTERY, MASS_MYSTERY_ERR).unwrap();

    let dist_𝑐 =
        rand_distr::Normal::new(COEFFICIENT_RESTITUTION, COEFFICIENT_RESTITUTION_ERR).unwrap();

    let dist_x = rand_distr::Normal::new(X, POS_ERR).unwrap();
    let dist_y = rand_distr::Normal::new(Y, POS_ERR).unwrap();

    let dist_processing = rand_distr::Normal::new(PROCESSING, PROCESSING_ERR).unwrap();
    let dist_d = rand_distr::Normal::new(DISTANCE, DISTANCE_ERR).unwrap();

    let mut rng = rand::thread_rng();

    for i in 1.. {
        println!(
            "{} {} {}",
            ">>>".cyan().dimmed(),
            format!("trial #{}…", i).bright_cyan().bold(),
            format!("(press {} or {} to exit)", "ctrl+C".bold(), "ctrl+D".bold()).cyan()
        );

        let 𝑚_probe = perturb_𝑚.perturb(
            &mut rng,
            util::read_float(
                &mut rl,
                "probe mass (kg):                ",
                MASS_PROBE_MIN,
                MASS_PROBE_MAX,
            )?,
        );

        let 𝑠 = perturb_𝑠.perturb(
            &mut rng,
            util::read_float(&mut rl, "launch position (m):            ", S_MIN, S_MAX)?,
        );

        let 𝜃 = perturb_𝜃
            .perturb(
                &mut rng,
                util::read_float(&mut rl, "launch angle (° to horizontal): ", Θ_MIN, Θ_MAX)?,
            )
            .to_radians();

        let 𝑣 = perturb_𝑣
            .perturb(
                &mut rng,
                util::read_float(&mut rl, "launch speed (m/s):             ", V_MIN, V_MAX)?,
            )
            .max(V_MIN / 2.);

        let 𝑟_probe = dist_𝑟_probe.sample(&mut rng);
        let 𝑟_mystery = dist_𝑟_mystery.sample(&mut rng);
        let 𝑚_mystery = dist_𝑚_mystery.sample(&mut rng);

        let processing = dist_processing.sample(&mut rng);

        let 𝑐 = dist_𝑐.sample(&mut rng);

        let 𝒙_probe = util::Vec2 { x: 0., y: 𝑠 };
        let 𝒙_mystery = util::Vec2 {
            x: dist_x.sample(&mut rng),
            y: dist_y.sample(&mut rng),
        };

        let 𝑚_total = 𝑚_probe + 𝑚_mystery;
        let 𝑟_total = 𝑟_probe + 𝑟_mystery;

        let 𝒗̂ = util::Vec2::normal_from_angle(𝜃);
        let 𝒗 = 𝒗̂ * 𝑣;

        // impact parameter, anchored at mystery disk center
        let 𝒃 = (𝒙_probe - 𝒙_mystery).project_onto(&𝒗.rot90());

        if 𝒃.norm() >= 𝑟_total {
            std::thread::sleep(std::time::Duration::from_secs_f64(
                dist_d.sample(&mut rng) / 𝑣 + processing,
            ));
            println!("{}\n", "  swing and a miss!".yellow());
            continue;
        }

        let δ𝒙 = 𝒃 - 𝒗̂ * (𝑟_total * 𝑟_total - 𝒃.norm2()).sqrt();

        let 𝒗_impact = 𝒗.project_onto(&δ𝒙);
        let 𝒗_probe = (𝒗 - 𝒗_impact) + 𝒗_impact * ((𝑚_probe - 𝑐 * 𝑚_mystery) / 𝑚_total);

        println!("{}", "  running…".green());

        // wait for ball to collide
        std::thread::sleep(std::time::Duration::from_secs_f64(
            (𝒙_mystery + δ𝒙).norm() / 𝑣 + processing,
        ));

        println!(
            "  {}\n  probe speed: {}\n  probe angle: {}\n",
            "you got a hit!".bright_green(),
            format!("{} m/s", 𝒗_probe.norm()).bold(),
            format!("{} ° to horizontal", 𝒗_probe.angle().to_degrees()).bold()
        );
    }

    Ok(())
}
