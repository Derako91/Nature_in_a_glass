"""Zusätzliche Materialvorlagen für die Biome, über lokales ComfyUI."""
import generate_sources as g

g.OUT=g.ROOT/'art/nature/comfyui-biomes'
g.STYLE=('Original finely textured pixel painting for a warm early 1990s settlement simulation. '
         'Top down flat tileable material sample, small natural texture, even diffuse light, no shadows, '
         'no objects, no border, no perspective, fills the whole image. ')
g.SUBJECTS={
    'dry_soil':'Dry ochre brown earth with sparse fine irregular cracks and tiny granular texture. Subtle muted earthy tones. No stones, no plants, no dunes.',
    'wet_soil':'Dark damp umber soil, fine compact silt and softly mottled earth texture. Muted brown and olive undertones. No plants, no pebbles, no standing water.',
    'highland_rock':'Weathered grey brown bedrock with small angular strata and subtle fissures, quiet natural mineral texture. No isolated boulders, no plants, no snow.',
}

if __name__=='__main__': g.main()
