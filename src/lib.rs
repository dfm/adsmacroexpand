use serde::Deserialize;
use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};
use walkdir::{DirEntry, WalkDir};
use yaml_front_matter::YamlFrontMatter;

pub fn find_bibliography_file<P: AsRef<Path>>(root: P) -> Option<PathBuf> {
    WalkDir::new(root).into_iter().find_map(paper_md)
}

pub fn update_bibliography_file<I: io::Read, O: io::Write>(input: &mut I, output: &mut O) {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();

    let mut mappings = MAPPINGS.to_vec();
    mappings.sort_by_key(|a| a.0.len());
    mappings
        .iter()
        .rev()
        .for_each(|(from, to)| buf = buf.replace(from, to));

    output.write_all(buf.as_bytes()).unwrap();
}

fn paper_md<E>(entry: Result<DirEntry, E>) -> Option<PathBuf> {
    let entry = entry.ok()?;
    let path = entry.path();
    if path.file_name()? != OsStr::new("paper.md") {
        return None;
    }
    let metadata = load_paper_metadata(path)?;
    Some(path.parent()?.join(metadata.bibliography))
}

fn load_paper_metadata<P: AsRef<Path>>(path: P) -> Option<Metadata> {
    let data = fs::read_to_string(path).ok()?;
    let metadata = YamlFrontMatter::parse::<Metadata>(&data).ok()?.metadata;
    Some(metadata)
}

#[derive(Deserialize)]
struct Metadata {
    bibliography: String,
}

// ADS Journal macros
// ref: https://ui.adsabs.harvard.edu/help/actions/journal-macros
const MAPPINGS: &[(&str, &str)] = &[
    (r"\aas", "American Astronomical Society Meeting Abstracts"),
    (r"\aj", "Astronomical Journal"),
    (r"\actaa", "Acta Astronomica"),
    (r"\araa", "Annual Review of Astron and Astrophysis"),
    (r"\apj", "Astrophysical Journal"),
    (r"\apjl", "Astrophysical Journal, Letters"),
    (r"\apjs", "Astrophysical Journal, Supplement"),
    (r"\ao", "Applied Optics"),
    (r"\apss", "Astrophysics and Space Science"),
    (r"\aap", "Astronomy and Astrophysics"),
    (r"\aapr", "Astronomy and Astrophysics Reviews"),
    (r"\aaps", "Astronomy and Astrophysics, Supplement"),
    (r"\aplett", "Astrophysics Letters"),
    (r"\apspr", "Astrophysics Space Physics Research"),
    (r"\azh", "Astronomicheskii Zhurnal"),
    (r"\baas", "Bulletin of the AAS"),
    (
        r"\bain",
        "Bulletin Astronomical Institute of the Netherlands",
    ),
    (r"\caa", "Chinese Astronomy and Astrophysics"),
    (r"\cjaa", "Chinese Journal of Astronomy and Astrophysics"),
    (
        r"\dps",
        "American Astronomical Society/Division for Planetary Sciences Meeting Abstracts",
    ),
    (r"\fcp", "Fundamental Cosmic Physics"),
    (r"\gca", "Geochimica Cosmochimica Acta"),
    (r"\grl", "Geophysics Research Letters"),
    (r"\iaucirc", "IAU Cirulars"),
    (r"\icarus", "Icarus"),
    (
        r"\jaavso",
        "Journal of the American Association of Variable Star Observers",
    ),
    (r"\jcap", "Journal of Cosmology and Astroparticle Physics"),
    (r"\jcp", "Journal of Chemical Physics"),
    (r"\jgr", "Journal of Geophysics Research"),
    (
        r"\jqsrt",
        "Journal of Quantitiative Spectroscopy and Radiative Transfer",
    ),
    (r"\jrasc", "Journal of the RAS of Canada"),
    (r"\maps", "Meteoritics and Planetary Science"),
    (r"\memras", "Memoirs of the RAS"),
    (r"\memsai", "Mem. Societa Astronomica Italiana"),
    (r"\mnras", "Monthly Notices of the RAS"),
    (r"\na", "New Astronomy"),
    (r"\nar", "New Astronomy Review"),
    (r"\nat", "Nature"),
    (r"\nphysa", "Nuclear Physics A"),
    (r"\pasa", "Publications of the Astron. Soc. of Australia"),
    (r"\pasp", "Publications of the ASP"),
    (r"\pasj", "Publications of the ASJ"),
    (r"\physrep", "Physics Reports"),
    (r"\physscr", "Physica Scripta"),
    (r"\planss", "Planetary Space Science"),
    (r"\pra", "Physical Review A: General Physics"),
    (r"\prb", "Physical Review B: Solid State"),
    (r"\prc", "Physical Review C"),
    (r"\prd", "Physical Review D"),
    (r"\pre", "Physical Review E"),
    (r"\prl", "Physical Review Letters"),
    (r"\procspie", "Proceedings of the SPIE"),
    (r"\psj", "Planetary Science Journal"),
    (r"\qjras", "Quarterly Journal of the RAS"),
    (r"\rmxaa", "Revista Mexicana de Astronomia y Astrofisica"),
    (r"\skytel", "Sky and Telescope"),
    (r"\solphys", "Solar Physics"),
    (r"\sovast", "Soviet Astronomy"),
    (r"\ssr", "Space Science Reviews"),
    (r"\zap", "Zeitschrift fuer Astrophysik"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_paper_metadata() {
        let path = find_bibliography_file("tests/data/demo");
        assert_eq!(path, Some(PathBuf::from("tests/data/demo/paper.bib")));
    }

    #[test]
    fn test_load_paper_metadata_subdir() {
        let path = find_bibliography_file("tests/data/subdir");
        assert_eq!(
            path,
            Some(PathBuf::from("tests/data/subdir/subdir/joss.bib"))
        );
    }
}
