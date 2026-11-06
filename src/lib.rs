use ::tilvisan as wrapped_tilvisan;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use std::path::PathBuf;

pyo3::create_exception!(xmpkit, AutohintError, PyException);

#[pymodule]
mod tilvisan {
    use super::*;

    #[pymodule_export]
    use AutohintError;

    // #[pyclass(from_py_object)]
    // #[derive(Clone)]
    // enum StemWidthMode {
    //     Natural,
    //     Quantized,
    //     Strong,
    // }

    // impl StemWidthMode {
    //     fn wrap(wrapped: wrapped_tilvisan::StemWidthMode) -> Self {
    //         match wrapped {
    //             wrapped_tilvisan::StemWidthMode::Natural => Self::Natural,
    //             wrapped_tilvisan::StemWidthMode::Quantized => Self::Quantized,
    //             wrapped_tilvisan::StemWidthMode::Strong => Self::Strong,
    //         }
    //     }

    //     fn unwrap(self) -> wrapped_tilvisan::StemWidthMode {
    //         match self {
    //             Self::Natural => wrapped_tilvisan::StemWidthMode::Natural,
    //             Self::Quantized => wrapped_tilvisan::StemWidthMode::Quantized,
    //             Self::Strong => wrapped_tilvisan::StemWidthMode::Strong,
    //         }
    //     }
    // }

    // #[pymethods]
    // impl StemWidthMode {
    //     fn to_word(&self) -> i32 {
    //         match self {
    //             StemWidthMode::Natural => -100,
    //             StemWidthMode::Quantized => 0,
    //             StemWidthMode::Strong => 100,
    //         }
    //     }
    // }

    // #[pyclass(from_py_object)]
    // #[derive(Clone)]
    // struct StemWidthModes {
    //     #[pyo3(get, set)]
    //     gray: StemWidthMode,
    //     #[pyo3(get, set)]
    //     gdi_cleartype: StemWidthMode,
    //     #[pyo3(get, set)]
    //     dw_cleartype: StemWidthMode,
    // }

    // #[pymethods]
    // impl StemWidthModes {
    //     #[new]
    //     #[pyo3(signature = (
    //         gray=StemWidthMode::Quantized,
    //         gdi_cleartype=StemWidthMode::Strong,
    //         dw_cleartype=StemWidthMode::Quantized,
    //     ))]
    //     fn new(
    //         gray: StemWidthMode,
    //         gdi_cleartype: StemWidthMode,
    //         dw_cleartype: StemWidthMode,
    //     ) -> Self {
    //         Self {
    //             gray,
    //             gdi_cleartype,
    //             dw_cleartype,
    //         }
    //     }
    // }

    #[pyfunction]
    #[pyo3(signature = (
        input,
        output=None,
        // stem_width_mode=StemWidthModes{
        //     gray:StemWidthMode::Quantized,
        //     gdi_cleartype:StemWidthMode::Strong,
        //     dw_cleartype:StemWidthMode::Quantized,
        // },
        composites=false,
        dehint=false,
        default_script=String::from("latn"),
        fallback_script=String::from("latn"),
        family_suffix=String::new(),
        hinting_limit=200,
        fallback_stem_width=0,
        ignore_restrictions=false,
        detailed_info=false,
        hinting_range_min=8,
        hinting_range_max=50,
        control_file=None,
        no_info=false,
        pre_hinting=false,
        adjust_subglyphs=false,
        reference=None,
        fallback_scaling=false,
        symbol=false,
        ttfa_table=false,
        ttfa_info=false,
        windows_compatibility=false,
        increase_x_height=14,
        x_height_snapping_exceptions=String::new(),
        reference_index=0,
        debug=false,
        epoch=None,
    ))]
    fn autohint(
        input: PathBuf,
        output: Option<PathBuf>,
        // stem_width_mode: StemWidthModes,
        composites: bool,
        dehint: bool,
        default_script: String,
        fallback_script: String,
        family_suffix: String,
        hinting_limit: u32,
        fallback_stem_width: u32,
        ignore_restrictions: bool,
        detailed_info: bool,
        hinting_range_min: u32,
        hinting_range_max: u32,
        control_file: Option<PathBuf>,
        no_info: bool,
        pre_hinting: bool,
        adjust_subglyphs: bool,
        reference: Option<PathBuf>,
        fallback_scaling: bool,
        symbol: bool,
        ttfa_table: bool,
        ttfa_info: bool,
        windows_compatibility: bool,
        increase_x_height: u32,
        x_height_snapping_exceptions: String,
        reference_index: u32,
        debug: bool,
        epoch: Option<u64>,
    ) -> PyResult<Vec<u8>> {
        let default_script_index =
            match wrapped_tilvisan::ScriptClassIndex::from_tag(&default_script) {
                Ok(v) => Ok(v),
                Err(error) => Err(AutohintError::new_err(error)),
            };
        let fallback_script_index =
            match wrapped_tilvisan::ScriptClassIndex::from_tag(&fallback_script) {
                Ok(v) => Ok(v),
                Err(error) => Err(AutohintError::new_err(error)),
            };
        let out = match output {
            Some(ref v) => match v.clone().into_string() {
                Ok(s) => s,
                _ => String::from("-"),
            },
            _ => String::from("-"),
        };
        match wrapped_tilvisan::autohint(&wrapped_tilvisan::Args {
            input: match input.into_string() {
                Ok(v) => v,
                _ => String::from("-"),
            },
            output: out.clone(),
            stem_width_mode: wrapped_tilvisan::StemWidthModes::default(),
            composites,
            dehint,
            default_script: default_script_index?,
            fallback_script: fallback_script_index?,
            family_suffix,
            fallback_stem_width,
            ignore_restrictions,
            detailed_info,
            hinting_range_min,
            hinting_range_max,
            hinting_limit,
            control_file,
            no_info,
            pre_hinting,
            adjust_subglyphs,
            reference,
            fallback_scaling,
            symbol,
            ttfa_table,
            ttfa_info,
            windows_compatibility,
            increase_x_height,
            x_height_snapping_exceptions,
            reference_index,
            debug,
            epoch,
        }) {
            Ok(v) => {
                if !output.is_none() {
                    std::fs::write(&out, &v).map_err(|e| {
                        AutohintError::new_err(format!(
                            "Error: Can't write output file '{}': {}",
                            out,
                            e.to_string(),
                        ))
                    })?;
                }
                Ok(v)
            }
            Err(error) => Err(AutohintError::new_err(error.to_string())),
        }
    }
}
