use anyhow::Result;
use ci_generate;
use sha2::{Digest, Sha256};
use std::{fs::File, io::Read};

static YARNHASH: &[&str] = &[
    "4ddd0a7e1e16c2750d0fb5b7ef9bb941a5550bcab254c2cdf55e97685822a282",
    "0e9741cd8e316ff9d013249170e8c07a548dbbefdb5ec3d66348a83d318e6c07",
    "c274f80372d90c012937370f0e1f15087d22e308ef98b27cea5dc0d2d088366c",
    "8af07bcc19724d7da8640a65ad2fe2ea6c95202d75b9069b2ee8130932cfe596",
    "1636d83f503297df43f0973219ae804c77f892d1a0ec67efea26438fa6de8229",
];

static POETRYHASH: &[&str] = &[
    "68d0db0f7cd1c9b3be3bb06741c32df87e4c5c21d246542af0e1ee43705032ce",
    "e57661eae3b38c320e9d8f214a505a75d53e745f4dffd8c38a23c1e792008d20",
    "5f750b77fdf8b9f7dbdb200ac25fa12b01dfafb883f3f5eaa119cafbdeb37a04",
    "32027b0c0cd8cb061f9055af36731c36dbe29941ff5d8caf05f91a0062db1371",
    "ff7cb60cea68e49d7bed71d6a8f3ed5d0a7842412def74acaf96502fcb2824d8",
    "32027b0c0cd8cb061f9055af36731c36dbe29941ff5d8caf05f91a0062db1371",
    "6d0e4915ea58c380c67b0b9e9939565d77db849ca8779b046a276764e381ff08",
    "b8a218ba82119d29001be3d139980dc50b343597da5cacf4b8d476dca6f72c43",
    "5f750b77fdf8b9f7dbdb200ac25fa12b01dfafb883f3f5eaa119cafbdeb37a04",
    "438b703e12a2010bdb51af9bbc6863c16bdc1f1735ee5816cd2a90d1d2683f42",
    "64c99914b0f49e4288ec46e21529fd17f04d9bad06c570852894b646c4cf2b78",
];

static MESONHASH: &[&str] = &[
    "0f68bf482a2136cd0111d664278648131f1d4a0cfeebc8e9239f4e4949f25056",
    "3862c5d9b405e1f0f10f6301370dcda6282702b1816dd3669253ca4075d0cbdf",
    "2a5d24edd7275ae2ce81e5ae4d66b849d979a671761a1d3bcc714f827b04a70e",
    "b0af3c6bbd96fc7fdc62cceb1736659baa28ef4d0ae2b6658fa41595bc2998bd",
    "8bf20ca1ba23894a63f06535e1514e5f1c6d8702f4a2b6798de4567c2adb6852",
    "203cf2b89512bf078493fcb49bf24eda1726ac7ea35d3f61d4ff17d602a73a62",
    "eaa4579ac024ea383555553e6af6801e464f38690dbe3a0d3b6022b675a34a4b",
    "12341e8823d91fd60293535233a44efee6ea016a2a6f68fa220ce060ff67b361",
    "5d40a9fd917a4ddd3d70cedbd5b7648d50c508edbaff8ea544dda6fdefcbb196",
    "32bef18582ca06c34df17e92243762b276161bfcbad2edfebd7742ae5ea6d649",
    "da877c597d391571f6022243c0ece0661cc6f75c0abf407e05b01c4959160cfc",
    "ac08cf2143b47acf636f41a9e340803865a00e16b2f72968d4f9e265e666b5af",
    "96a54d08c42cabb4a2b47cb2910e9c50d1046ca54364c7f80a3141dfd33b80c9",
    "d35872638c9ff5009579d04c118f71599356d14a8520b09a41d6085f1261535b",
    "d5a1883706d1d26c89f8ac04f1df8410aee834ed08d17308f75194732dbeba91",
];

static MAVENHASH: &[&str] = &[
    "1ff6e00ba8636fc9adff59ad0aa0449c6050a4ff43d59359224931c9144a26be",
    "e57661eae3b38c320e9d8f214a505a75d53e745f4dffd8c38a23c1e792008d20",
    "5f750b77fdf8b9f7dbdb200ac25fa12b01dfafb883f3f5eaa119cafbdeb37a04",
    "eb2443768e732b0e613871c78cb19fd0cd0dcf133591566fb6e92bb777d6eb6b",
    "3a1c9b3076267257e4cb7fd3e9820d62fa843b74aab3f0331e64b85178881f43",
    "cf51d282304cca4c0588fec8ff6f1d96e461e8d6871b1f4de66b57402e5880f1",
    "5f750b77fdf8b9f7dbdb200ac25fa12b01dfafb883f3f5eaa119cafbdeb37a04",
    "b94bae728be16b62cbef7b2f311334d7d57cb21e269e015f1d2f657eaf54504c",
    "64c99914b0f49e4288ec46e21529fd17f04d9bad06c570852894b646c4cf2b78",
];

static CARGOHASH: &[&str] = &[
    "3d741e6841556ec11c0817840a6d3c34456608001d76a2f1237b84cf541af0c7",
    "537ee56f8daa7065ded0b5c52680a65cecf566e0ce3da8bce8fa945d9859ffb4",
    "26712e4fc7ac4040e72ccffae7b68f16e4e6b534fa43273c33f3638deaaa51b9",
    "1bf5bdbff789285b7e47685b5df1613ae9040218ad2696291ac8a8ba2ea3901e",
    "86d6865f97c4d022c8f183f25a9ba3d473fe272f6ac7a0989a46ba59fa7eb261",
    "11604eb7ed4df8e3d0e7a64525d9f5e466a54d4ee44edc598c198984fff537ff",
    "6e56a425dad4c4536b10f930f63dcfd872d8a52f4ad6fa75498e125fff287dbd",
    "e62f66e68325d5174c921ba24dc1a62ac55661876c5b7b5cdd320579971d69d2",
    "16645c1b1b7c017e219363f3820a85c4aae6d0108466045b698a4d08b8e72554",
    "d126070e53a97387b339cc1b4c794e9bc134df061d6cb5ab37cef0bfe986228a",
    "e7457b6c88b3f7bf2a1e17de608ebe2e239e146a460378c28eb87a3fd12cb5ea",
];

pub fn hash_comparation(hash_file: String, hash_expected: &str) -> bool {
    match hash_expected {
        "yarn" => YARNHASH.contains(&hash_file.as_str()),
        "poetry" => POETRYHASH.contains(&hash_file.as_str()),
        "meson" => MESONHASH.contains(&hash_file.as_str()),
        "maven" => MAVENHASH.contains(&hash_file.as_str()),
        "cargo" => CARGOHASH.contains(&hash_file.as_str()),
        _ => false,
    }
}

pub fn compute_sha256_hash_if_file(file: &str) -> Result<String> {
    let mut file = File::open(file)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn create_yarn_project() -> Result<()> {
    ci_generate::CreateCi::create_ci(
        &ci_generate::toolchain::yarn::Yarn::new(),
        "",
        std::path::Path::new("~/Documenti/GitHub/ci-generate/tests/output/yarn_project"),
        "Apache-2.0",
        "main",
    )
}

pub fn create_poetry_project() -> Result<()> {
    ci_generate::CreateProject::create_project(
        &ci_generate::toolchain::poetry::Poetry::new(),
        "Myprog",
        std::path::Path::new("~/Documenti/GitHub/ci-generate/tests/output/poetry_project"),
        "BSD-1-Clause",
        "master",
    )
}

pub fn create_meson_project() -> Result<()> {
    ci_generate::CreateProject::create_project(
        &ci_generate::toolchain::meson::Meson::new(ci_generate::toolchain::meson::ProjectKind::C),
        "",
        std::path::Path::new("~/Documenti/GitHub/ci-generate/tests/output/meson_project"),
        "APL-1.0",
        "main",
    )
}

pub fn create_maven_project() -> Result<()> {
    ci_generate::CreateProject::create_project(
        &ci_generate::toolchain::maven::Maven::new("POL"),
        "Myprog",
        std::path::Path::new("~/Documenti/GitHub/ci-generate/tests/output/maven_project"),
        "BSD-1-Clause",
        "master",
    )
}

pub fn create_cargo_project() -> Result<()> {
    ci_generate::CreateCi::create_ci(
        &ci_generate::toolchain::cargo::Cargo::new("Docker-image"),
        "Project",
        std::path::Path::new("~/Documenti/GitHub/ci-generate/tests/output/cargo_project"),
        "EUPL-1.2",
        "main",
    )
}
