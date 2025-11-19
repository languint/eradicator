use eradicator_core::strategy::StrategyParser;

mod state;

fn main() -> Result<(), String> {
    let mut parser = StrategyParser::new(
        r#"mode=Easy
map=Baseplate
loadout=Shotgunner, Juggernaut, EDJ
---
Shotgunner 100 1000
Shotgunner 100 1292
Juggernaut 422 2041
Juggernaut 232 2311
EDJ 312 120
---
place 1
place 2
upgrade 1 BOTTOM
upgrade 2 BOTTOM
@200{ upgrade 1 TOP }
@200{ upgrade 2 TOP }
@375{ upgrade 1 TOP }
@1950{ upgrade 1 TOP }
@2400{ upgrade 1 TOP }
@325{upgrade 1 BOTTOM }
@375{ upgrade 2 TOP }
@1950{ upgrade 2 TOP }
@2400{ upgrade 2 TOP }
@325{upgrade 2 BOTTOM }"#,
    );

    println!(
        "{:?}",
        parser.parse().expect("Expected parsing to succeed!")
    );

    Ok(())
}
