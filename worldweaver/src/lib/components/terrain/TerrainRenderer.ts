import REGL from 'regl';
import type { ViewTransform, TerrainConfig } from './types';

export class TerrainRenderer {
  private regl: REGL.Regl;
  private canvas: HTMLCanvasElement;
  private heightmapTexture: REGL.Texture2D | null = null;
  private colorRampTexture: REGL.Texture2D | null = null;
  private flowTexture: REGL.Texture2D | null = null;
  private drawTerrain: REGL.DrawCommand | null = null;
  private config: TerrainConfig | null = null;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.regl = REGL({
      canvas,
      attributes: {
        antialias: false,
        depth: false,
        stencil: false,
        alpha: false
      },
      extensions: [
        'OES_texture_float',
        'OES_texture_float_linear',
        'OES_standard_derivatives'
      ],
      optionalExtensions: [
        'OES_texture_float',
        'OES_texture_float_linear',
        'OES_standard_derivatives'
      ]
    });

    this.initializeColorRamp();
  }

  private initializeColorRamp() {
    // Create hypsometric color ramp (256×1 RGBA texture)
    // Google Maps-style realistic terrain colors
    const colorStops = [
      { elev: 0.00, rgb: [4, 30, 66] },      // deep ocean (dark navy)
      { elev: 0.35, rgb: [26, 84, 144] },    // ocean
      { elev: 0.45, rgb: [43, 123, 185] },   // shallow water
      { elev: 0.48, rgb: [136, 201, 240] },  // very shallow
      { elev: 0.50, rgb: [245, 230, 200] },  // beach/sand
      { elev: 0.52, rgb: [212, 231, 176] },  // coastal lowlands
      { elev: 0.58, rgb: [184, 216, 139] },  // low plains
      { elev: 0.65, rgb: [154, 199, 119] },  // plains
      { elev: 0.72, rgb: [122, 181, 92] },   // foothills
      { elev: 0.78, rgb: [201, 184, 150] },  // lower mountains (brown)
      { elev: 0.84, rgb: [179, 154, 125] },  // mid mountains
      { elev: 0.88, rgb: [157, 138, 114] },  // high mountains
      { elev: 0.92, rgb: [139, 125, 107] },  // very high
      { elev: 0.96, rgb: [212, 207, 201] },  // snow line
      { elev: 1.00, rgb: [255, 255, 255] }   // peaks
    ];

    const rampData = new Uint8Array(256 * 4);
    for (let i = 0; i < 256; i++) {
      const elev = i / 255;
      
      // Find color stops to interpolate between
      let lower = colorStops[0];
      let upper = colorStops[colorStops.length - 1];
      
      for (let j = 0; j < colorStops.length - 1; j++) {
        if (elev >= colorStops[j].elev && elev <= colorStops[j + 1].elev) {
          lower = colorStops[j];
          upper = colorStops[j + 1];
          break;
        }
      }

      // Interpolate
      const t = (elev - lower.elev) / (upper.elev - lower.elev);
      const r = Math.round(lower.rgb[0] * (1 - t) + upper.rgb[0] * t);
      const g = Math.round(lower.rgb[1] * (1 - t) + upper.rgb[1] * t);
      const b = Math.round(lower.rgb[2] * (1 - t) + upper.rgb[2] * t);

      rampData[i * 4 + 0] = r;
      rampData[i * 4 + 1] = g;
      rampData[i * 4 + 2] = b;
      rampData[i * 4 + 3] = 255;
    }

    this.colorRampTexture = this.regl.texture({
      width: 256,
      height: 1,
      data: rampData,
      format: 'rgba',
      type: 'uint8',
      min: 'linear',
      mag: 'linear',
      wrapS: 'clamp',
      wrapT: 'clamp'
    });
  }

  public setConfig(config: TerrainConfig) {
    this.config = config;
  }

  public updateFlowData(width: number, height: number, data: Uint8Array) {
    if (this.flowTexture) {
      this.flowTexture.destroy();
    }

    // Create R8 texture for flow accumulation
    this.flowTexture = this.regl.texture({
      width,
      height,
      data,
      format: 'luminance',
      type: 'uint8',
      min: 'linear',
      mag: 'linear',
      wrap: 'clamp'
    });
  }

  public updateHeightmap(width: number, height: number, data: Float32Array) {
    if (this.heightmapTexture) {
      this.heightmapTexture.destroy();
    }

    // Try to use float texture, fallback to RGBA if not supported
    try {
        this.heightmapTexture = this.regl.texture({
          width,
          height,
          data,
          format: 'luminance',
          type: 'float',
          min: 'linear',  // Bilinear filtering reduces grain
          mag: 'linear',
          wrapS: 'clamp',
          wrapT: 'clamp'
        });
    } catch (e) {
      console.warn('Float textures not supported, using RGBA fallback');
      // Convert float data to RGBA uint8
      const rgbaData = new Uint8Array(width * height * 4);
      for (let i = 0; i < data.length; i++) {
        const value = Math.floor(data[i] * 255);
        rgbaData[i * 4 + 0] = value;
        rgbaData[i * 4 + 1] = value;
        rgbaData[i * 4 + 2] = value;
        rgbaData[i * 4 + 3] = 255;
      }
      
        this.heightmapTexture = this.regl.texture({
          width,
          height,
          data: rgbaData,
          format: 'rgba',
          type: 'uint8',
          min: 'linear',  // Bilinear filtering reduces grain
          mag: 'linear',
          wrapS: 'clamp',
          wrapT: 'clamp'
        });
    }
  }

  public updateHeightmapRegion(
    x: number,
    y: number,
    width: number,
    height: number,
    data: Float32Array
  ) {
    if (this.heightmapTexture) {
      this.heightmapTexture.subimage({
        data,
        x,
        y,
        width,
        height
      });
    }
  }

  public render(
    transform: ViewTransform, 
    sunAngle: number = 45, 
    contourInterval: number = 100, 
    hideUnderwater: boolean = false,
    showRivers: boolean = false,
    riverThreshold: number = 1000
  ) {
    if (!this.heightmapTexture || !this.colorRampTexture || !this.config) {
      return;
    }

    if (!this.drawTerrain) {
      this.createDrawCommand();
    }

    const sunAngleRad = (sunAngle * Math.PI) / 180;
    const sunDir = [
      Math.cos(sunAngleRad),
      Math.sin(sunAngleRad),
      0.5
    ];

    // Normalize sun direction
    const len = Math.sqrt(sunDir[0] * sunDir[0] + sunDir[1] * sunDir[1] + sunDir[2] * sunDir[2]);
    sunDir[0] /= len;
    sunDir[1] /= len;
    sunDir[2] /= len;

    this.regl.clear({
      color: [0.1, 0.15, 0.3, 1.0],
      depth: 1
    });

    if (this.drawTerrain) {
      this.drawTerrain({
        heightmap: this.heightmapTexture,
        colorRamp: this.colorRampTexture,
        flowData: this.flowTexture || this.colorRampTexture, // Fallback if no flow data
        resolution: [this.config.world_width, this.config.world_height],
        canvasSize: [this.canvas.width, this.canvas.height],
        sunDir,
        contourInterval,
        majorContourEvery: 5,
        maxElevation: this.config.max_elevation,
        viewTransform: this.getViewMatrix(transform),
        seaLevel: this.config.sea_level,
        hideUnderwater: hideUnderwater ? 1.0 : 0.0,
        showRivers: showRivers ? 1.0 : 0.0,
        riverThreshold: riverThreshold
      });
    }
  }

  private createDrawCommand() {
    this.drawTerrain = this.regl({
      vert: `
        precision highp float;
        attribute vec2 position;
        void main() {
          gl_Position = vec4(position, 0.0, 1.0);
        }
      `,

      frag: `
        #extension GL_OES_standard_derivatives : enable
        precision highp float;
        uniform sampler2D heightmap;
        uniform sampler2D colorRamp;
        uniform sampler2D flowData;
        uniform vec2 resolution;
        uniform vec2 canvasSize;
        uniform vec3 sunDir;
        uniform float contourInterval;
        uniform float majorContourEvery;
        uniform float maxElevation;
        uniform mat3 viewTransform;
        uniform float seaLevel;
        uniform float hideUnderwater;
        uniform float showRivers;
        uniform float riverThreshold;

        void main() {
          // Apply view transform (pan/zoom) to screen coordinates
          vec2 screenPos = gl_FragCoord.xy;
          
          // Transform screen position to world space using inverse view matrix
          // viewTransform is [scale*cos, scale*sin, 0, -scale*sin, scale*cos, 0, tx, ty, 1]
          // We need to apply inverse transform: (screenPos - translate) / scale
          vec2 translate = vec2(viewTransform[2][0], viewTransform[2][1]);
          float scale = viewTransform[0][0]; // Assuming no rotation for now
          
          // Transform to world coordinates
          vec2 worldPos = (screenPos - translate) / scale;
          
          // Normalize to UV coordinates (0-1 range)
          vec2 uv = worldPos / canvasSize;
          
          // Clamp to valid texture range
          if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
            gl_FragColor = vec4(0.05, 0.08, 0.15, 1.0); // Deep ocean outside bounds
            return;
          }
          
          // Sample elevation with proper texel size
          vec2 texel = 1.0 / resolution;
          float h  = texture2D(heightmap, uv).r;
          
          // Hide underwater terrain if enabled
          if (hideUnderwater > 0.5 && h < seaLevel) {
            gl_FragColor = vec4(0.05, 0.08, 0.15, 1.0); // Deep ocean color
            return;
          }
          
          float hN = texture2D(heightmap, uv + vec2(0.0, texel.y)).r;
          float hS = texture2D(heightmap, uv - vec2(0.0, texel.y)).r;
          float hE = texture2D(heightmap, uv + vec2(texel.x, 0.0)).r;
          float hW = texture2D(heightmap, uv - vec2(texel.x, 0.0)).r;

          // Horn's method for better gradient estimation (reduces grain)
          float dzdx = ((hE + 2.0 * hE + hE) - (hW + 2.0 * hW + hW)) / 8.0;
          float dzdy = ((hN + 2.0 * hN + hN) - (hS + 2.0 * hS + hS)) / 8.0;
          
          // Compute surface normal with moderate vertical exaggeration
          float zFactor = 100.0; // Reduced from 200 for less grain
          vec3 normal = normalize(vec3(-dzdx * zFactor, -dzdy * zFactor, 1.0));

          // Multi-directional hillshade (GDAL-style)
          // Primary light: 315° azimuth (NW), 45° altitude
          vec3 light1 = normalize(vec3(-0.7071, 0.7071, 1.0));
          float shade1 = max(0.0, dot(normal, light1));
          
          // Secondary light: 225° azimuth (SW), 30° altitude (fill light)
          vec3 light2 = normalize(vec3(-0.7071, -0.7071, 0.5773));
          float shade2 = max(0.0, dot(normal, light2));
          
          // Combine lights with weights (primary 70%, fill 30%)
          float hillshade = shade1 * 0.7 + shade2 * 0.3;
          
          // Ambient occlusion approximation (darker in valleys)
          float ao = 1.0 - smoothstep(0.0, 0.3, length(vec2(dzdx, dzdy)));
          hillshade *= 0.85 + 0.15 * ao;
          
          // Final hillshade range: 0.3 to 1.0 (prevents pure black)
          hillshade = 0.3 + 0.7 * hillshade;

          // Hypsometric tinting
          vec3 hypsColor = texture2D(colorRamp, vec2(clamp(h, 0.0, 1.0), 0.5)).rgb;

          // Contour lines with proper anti-aliasing
          float elevMeters = h * maxElevation;
          float scaled = elevMeters / contourInterval;
          float df = fwidth(scaled) * 0.5; // Reduce fwidth influence for cleaner lines
          float contour = smoothstep(df, df * 3.0, abs(fract(scaled) - 0.5) * 2.0);

          float majorScaled = elevMeters / (contourInterval * majorContourEvery);
          float dfMajor = fwidth(majorScaled) * 0.5;
          float majorContour = smoothstep(dfMajor, dfMajor * 3.0,
            abs(fract(majorScaled) - 0.5) * 2.0);
          
          // Contour opacity (thinner lines)
          float contourAlpha = (1.0 - contour) * 0.3 + (1.0 - majorContour) * 0.15;

          // CRITICAL: Multiply blend mode (like QGIS)
          // Color layer * hillshade layer = final terrain
          vec3 terrain = hypsColor * hillshade;
          
          // Add contour lines on top
          vec3 contourColor = vec3(0.3, 0.25, 0.2);
          vec3 finalColor = mix(terrain, contourColor, contourAlpha);
          
          // Add rivers and lakes if enabled
          if (showRivers > 0.5 && h > seaLevel) {
            // Sample flow accumulation
            float flow = texture2D(flowData, uv).r;
            
            // Normalize flow threshold (0-1 range)
            float flowNorm = flow * 255.0; // Convert back from 0-255 range
            float threshold = riverThreshold / 5000.0; // Normalize threshold
            
            if (flowNorm > threshold) {
              // This is a river or lake
              // Width based on flow amount
              float riverWidth = smoothstep(threshold, threshold * 2.0, flowNorm);
              
              // River color (blue, gets darker with more flow)
              vec3 riverColor = mix(
                vec3(0.4, 0.7, 0.9),  // Light blue (streams)
                vec3(0.1, 0.3, 0.6),  // Dark blue (major rivers)
                riverWidth
              );
              
              // Blend river on top of terrain
              finalColor = mix(finalColor, riverColor, riverWidth * 0.8);
            }
          }

          gl_FragColor = vec4(finalColor, 1.0);
        }
      `,

      attributes: {
        position: [
          [-1, -1],
          [1, -1],
          [-1, 1],
          [1, 1]
        ]
      },

      uniforms: {
        heightmap: this.regl.prop<any, 'heightmap'>('heightmap'),
        colorRamp: this.regl.prop<any, 'colorRamp'>('colorRamp'),
        flowData: this.regl.prop<any, 'flowData'>('flowData'),
        resolution: this.regl.prop<any, 'resolution'>('resolution'),
        canvasSize: this.regl.prop<any, 'canvasSize'>('canvasSize'),
        sunDir: this.regl.prop<any, 'sunDir'>('sunDir'),
        contourInterval: this.regl.prop<any, 'contourInterval'>('contourInterval'),
        majorContourEvery: this.regl.prop<any, 'majorContourEvery'>('majorContourEvery'),
        maxElevation: this.regl.prop<any, 'maxElevation'>('maxElevation'),
        viewTransform: this.regl.prop<any, 'viewTransform'>('viewTransform'),
        seaLevel: this.regl.prop<any, 'seaLevel'>('seaLevel'),
        hideUnderwater: this.regl.prop<any, 'hideUnderwater'>('hideUnderwater'),
        showRivers: this.regl.prop<any, 'showRivers'>('showRivers'),
        riverThreshold: this.regl.prop<any, 'riverThreshold'>('riverThreshold')
      },

      primitive: 'triangle strip',
      count: 4
    });
  }

  private getViewMatrix(transform: ViewTransform): number[] {
    const { translateX, translateY, scale, rotation } = transform;
    const cos = Math.cos(rotation);
    const sin = Math.sin(rotation);

    // Inverse transform for shader
    const invScale = 1.0 / scale;
    return [
      invScale * cos, invScale * sin, 0,
      -invScale * sin, invScale * cos, 0,
      -translateX * invScale, -translateY * invScale, 1
    ];
  }

  public destroy() {
    if (this.heightmapTexture) {
      this.heightmapTexture.destroy();
    }
    if (this.colorRampTexture) {
      this.colorRampTexture.destroy();
    }
    this.regl.destroy();
  }
}
