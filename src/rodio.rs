use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // La sortie son de l'ordi : si elle est détruite, le son s'arrête
    let sortie = rodio::DeviceSinkBuilder::open_default_sink()?;
    let player = rodio::Player::connect_new(sortie.mixer());

    let fichier = File::open("src/musique.mp3")?;
    player.append(rodio::Decoder::try_from(fichier)?);

    player.sleep_until_end(); // attend la fin du morceau
    Ok(())
}//"Buffer size 2048 is not in the supported range 14..=1024"