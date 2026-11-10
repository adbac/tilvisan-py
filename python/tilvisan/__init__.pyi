from os import PathLike

from _typeshed import Incomplete

class AutohintError(Exception):
    """Error type for auto-hinting operations."""

def autohint(
    input: str | PathLike[str],
    output: str | PathLike[str] | None = None,
    composites: bool = False,
    dehint: bool = False,
    default_script: str = "latn",
    fallback_script: str = "latn",
    family_suffix: str = "",
    hinting_limit: int = 200,
    fallback_stem_width: int = 0,
    ignore_restrictions: bool = False,
    detailed_info: bool = False,
    hinting_range_min: int = 8,
    hinting_range_max: int = 50,
    control_file: str | PathLike[str] | None = None,
    no_info: bool = False,
    pre_hinting: bool = False,
    adjust_subglyphs: bool = False,
    reference: str | PathLike[str] | None = None,
    fallback_scaling: bool = False,
    symbol: bool = False,
    ttfa_table: bool = False,
    ttfa_info: bool = False,
    windows_compatibility: bool = False,
    increase_x_height: int = 14,
    x_height_snapping_exceptions: str = "",
    reference_index: int = 0,
    debug: bool = False,
    epoch: int | None = None,
) -> bytes:
    """Auto-hint a static or variable TrueType font.

    Args:
        input: Input font file path.
        input: Output font file path.
            If `None`, the hinted font is not written to the disk.
        composites: Hint composite glyphs separately.
        dehint: Remove all hints.
        default_script: The default script tag to use.
        fallback_script: The fallback script tag to use.
        family_suffix: The suffix to append to family name strings in the 'name' table.
        hinting_limit: The hinting limit (PPEM).
        fallback_stem_width: The value of the fallback stem width (font units).
        ignore_restrictions: If `True`, ignore font restrictions (fsType bit 1).
        detailed_info: If `True`, add detailed autohint info to 'name' table.
        hinting_range_min: The minimum hinting range (PPEM).
        hinting_range_max: The maximum hinting range (PPEM).
        control_file: The control file path.
        no_info: If `True`, do not add autohinter info to 'name' table.
        pre_hinting: If `True`, do pre-hinting (deprecated alias for adjust-subglyphs).
        adjust_subglyphs: If `True`, handle subglyph adjustments in uncommon fonts.
        reference: Reference font file path.
        fallback_scaling: If `True`, use fallback scaling instead of hinting.
        symbol: If `True`, the font will be processed as a symbol font.
        ttfa_table: If `True`, add TTFA table.
        ttfa_info: If `True`, show TTFA table from input font and exit.
            Currently not implement.
        windows_compatibility: If `True`, enable Windows compatibility
            (blue zones for usWinAscent/Descent).
        increase_x_height: The x-height increase limit (PPEM).
        x_height_snapping_exceptions: X-height snapping exceptions.
        reference_index: The reference font index.
        debug: If `True`, enable debug mode.
        epoch: The epoch for reproducible builds (seconds since 1970-01-01).

    Raises:
        AutohintError: The library's error type.
    """

def __getattr__(name: str) -> Incomplete: ...
