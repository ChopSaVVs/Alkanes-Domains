use serde_json::json;
use anyhow::{Error, Result};

pub struct SvgGenerator;

impl SvgGenerator {

  pub fn get_attributes(domain_name: &str) -> Result<String, Error> {
    let name_length = domain_name.chars().count();
    let attributes = json!({
      "name": domain_name,
      "length": name_length
    });

    Ok(attributes.to_string())
  }

  pub fn generate_svg(domain_name: &str) -> Result<String, Error> {
    let mut svg = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?><svg width=\"100%\" height=\"100%\" viewBox=\"0 0 270 270\" xmlns=\"http://www.w3.org/2000/svg\"><defs><radialGradient id=\"bgGradient\" cx=\"50%\" cy=\"50%\" r=\"80%\" fx=\"50%\" fy=\"50%\"><stop offset=\"0%\" style=\"stop-color:#18191a;stop-opacity:1\"/><stop offset=\"100%\" style=\"stop-color:#000000;stop-opacity:1\"/></radialGradient><linearGradient id=\"vGrad1\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"1\"><stop offset=\"0%\" stop-color=\"#00ff99\"/><stop offset=\"100%\" stop-color=\"#ff0066\"/></linearGradient><linearGradient id=\"vGrad2\" x1=\"0\" y1=\"1\" x2=\"1\" y2=\"0\"><stop offset=\"0%\" stop-color=\"#00ccff\"/><stop offset=\"100%\" stop-color=\"#ffff66\"/></linearGradient><linearGradient id=\"vGrad3\" x1=\"0\" y1=\"0\" x2=\"1\" y2=\"0\"><stop offset=\"0%\" stop-color=\"#ff6600\"/><stop offset=\"100%\" stop-color=\"#6600ff\"/></linearGradient><linearGradient id=\"vDotGradient\" x1=\"100%\" y1=\"0%\" x2=\"0%\" y2=\"0%\"><stop offset=\"0%\" stop-color=\"#2c6df5\"/><stop offset=\"16%\" stop-color=\"#d94259\"/><stop offset=\"32%\" stop-color=\"#f5c18e\"/><stop offset=\"48%\" stop-color=\"#a558e7\"/><stop offset=\"64%\" stop-color=\"#68e06d\"/><stop offset=\"80%\" stop-color=\"#ede8e7\"/><stop offset=\"100%\" stop-color=\"#8a6248\"/></linearGradient><filter id=\"dotGlow\" x=\"-30%\" y=\"-30%\" width=\"160%\" height=\"160%\"><feGaussianBlur stdDeviation=\"6\" result=\"coloredBlur\"/><feMerge><feMergeNode in=\"coloredBlur\"/><feMergeNode in=\"SourceGraphic\"/></feMerge></filter><filter id=\"textShadow\" x=\"-10%\" y=\"-10%\" width=\"120%\" height=\"120%\"><feDropShadow dx=\"0.5\" dy=\"1\" stdDeviation=\"0.7\" flood-color=\"#000\" flood-opacity=\"0.18\"/></filter><linearGradient id=\"paint0_linear_81_297\" x1=\"189\" y1=\"196.5\" x2=\"204.5\" y2=\"373\" gradientUnits=\"userSpaceOnUse\"><stop stop-color=\"#800E96\"/><stop offset=\"0.307692\" stop-color=\"#5645B4\"/><stop offset=\"0.639423\" stop-color=\"#2D70C8\"/><stop offset=\"1\" stop-color=\"#081828\"/></linearGradient><linearGradient id=\"paint1_linear_81_297\" x1=\"369.5\" y1=\"204.5\" x2=\"287.5\" y2=\"363.5\" gradientUnits=\"userSpaceOnUse\"><stop stop-color=\"#929DDE\"/><stop offset=\"0.307692\" stop-color=\"#B8AEB5\"/><stop offset=\"0.629808\" stop-color=\"#BE5131\"/><stop offset=\"1\" stop-color=\"#9F2502\"/></linearGradient><linearGradient id=\"paint2_linear_81_297\" x1=\"212.5\" y1=\"183.5\" x2=\"298.303\" y2=\"108.96\" gradientUnits=\"userSpaceOnUse\"><stop stop-color=\"#0898CD\"/><stop offset=\"0.673077\" stop-color=\"#B1C253\"/><stop offset=\"1\" stop-color=\"#82910A\"/></linearGradient><linearGradient id=\"paint3_linear_81_297\" x1=\"39.5\" y1=\"8.5\" x2=\"154.511\" y2=\"42.1806\" gradientUnits=\"userSpaceOnUse\"><stop stop-color=\"#77850E\"/><stop offset=\"0.451923\" stop-color=\"#35957C\"/><stop offset=\"1\" stop-color=\"#0798CD\"/></linearGradient><linearGradient id=\"paint4_linear_81_297\" x1=\"105.194\" y1=\"119\" x2=\"213.044\" y2=\"196.632\" gradientUnits=\"userSpaceOnUse\"><stop stop-color=\"#30841A\"/><stop offset=\"0.514423\" stop-color=\"#430224\"/><stop offset=\"1\" stop-color=\"#589469\"/></linearGradient><linearGradient id=\"paint5_linear_81_297\" x1=\"395.806\" y1=\"119\" x2=\"287.956\" y2=\"196.632\" gradientUnits=\"userSpaceOnUse\"><stop stop-color=\"#3E52EB\"/><stop offset=\"0.514423\" stop-color=\"#1004E8\"/><stop offset=\"1\" stop-color=\"#635FE5\"/></linearGradient><linearGradient id=\"paint6_linear_81_297\" x1=\"105.194\" y1=\"119\" x2=\"213.044\" y2=\"196.632\" gradientUnits=\"userSpaceOnUse\"><stop stop-color=\"#30841A\"/><stop offset=\"0.514423\" stop-color=\"#430224\"/><stop offset=\"1\" stop-color=\"#589469\"/></linearGradient><linearGradient id=\"paint7_linear_81_297\" x1=\"395.806\" y1=\"119\" x2=\"287.956\" y2=\"196.632\" gradientUnits=\"userSpaceOnUse\"><stop stop-color=\"#3E52EB\"/><stop offset=\"0.514423\" stop-color=\"#1004E8\"/><stop offset=\"1\" stop-color=\"#635FE5\"/></linearGradient><linearGradient id=\"paint8_linear_81_297\" x1=\"251.5\" y1=\"261.502\" x2=\"251.5\" y2=\"382\" gradientUnits=\"userSpaceOnUse\"><stop stop-color=\"#B66829\"/><stop offset=\"1\" stop-color=\"#B62F03\"/></linearGradient><linearGradient id=\"paint9_linear_81_297\" x1=\"249.5\" y1=\"261.502\" x2=\"249.5\" y2=\"382\" gradientUnits=\"userSpaceOnUse\"><stop stop-color=\"#B66829\"/><stop offset=\"1\" stop-color=\"#B62F03\"/></linearGradient></defs><rect width=\"100%\" height=\"100%\" fill=\"url(#bgGradient)\"/>");
    
    svg.push_str("<g transform=\"translate(20, 10) scale(0.2)\">");
    svg.push_str("<path d=\"M114 176.5L194 365C195.5 369.5 201.6 379.1 214 381.5C226.4 383.9 243 307.667 250.5 271L220.5 197L114 176.5Z\" fill=\"url(#paint0_linear_81_297)\"/>");
    svg.push_str("<path d=\"M387 176.5L307 365C305.5 369.5 299.4 379.1 287 381.5C274.6 383.9 258 307.667 250.5 271L280.5 197L387 176.5Z\" fill=\"url(#paint1_linear_81_297)\"/>");
    svg.push_str("<rect x=\"148\" y=\"119\" width=\"173\" height=\"83\" fill=\"url(#paint2_linear_81_297)\"/>");
    svg.push_str("<rect width=\"173\" height=\"83\" transform=\"matrix(-1 0 0 1 353 119)\" fill=\"url(#paint3_linear_81_297)\"/>");
    svg.push_str("<path d=\"M95.1782 131.455C92.7139 125.528 97.0694 119 103.489 119H180.999C184.617 119 187.884 121.167 189.291 124.5L216.724 189.5C219.229 195.434 214.873 202 208.433 202H130.331C126.682 202 123.394 199.796 122.007 196.421L106.63 159L95.1782 131.455Z\" fill=\"url(#paint4_linear_81_297)\"/>");
    svg.push_str("<path d=\"M405.822 131.455C408.286 125.528 403.931 119 397.511 119H320.001C316.383 119 313.116 121.167 311.709 124.5L284.276 189.5C281.771 195.434 286.127 202 292.567 202H370.669C374.318 202 377.606 199.796 378.993 196.421L394.37 159L405.822 131.455Z\" fill=\"url(#paint5_linear_81_297)\"/>");
    svg.push_str("<path d=\"M106.63 125L180 120.5L190 126.5L222.5 202H133L119 160L106.63 125Z\" fill=\"url(#paint6_linear_81_297)\"/>");
    svg.push_str("<path d=\"M394.37 125L321 120.5L311 126.5L278.5 202H368L382 160L394.37 125Z\" fill=\"url(#paint7_linear_81_297)\"/>");
    svg.push_str("<path d=\"M250.5 271L288.839 366.632C290.903 371.78 291.935 374.354 291.506 376.42C291.132 378.227 290.059 379.814 288.521 380.834C286.762 382 283.989 382 278.443 382H224.281C218.816 382 216.083 382 214.334 380.852C212.804 379.847 211.728 378.283 211.337 376.495C210.89 374.451 211.867 371.899 213.822 366.795L250.5 271Z\" fill=\"url(#paint8_linear_81_297)\"/>");
    svg.push_str("<path d=\"M250.5 271L212.161 366.632C210.097 371.78 209.065 374.354 209.494 376.42C209.868 378.227 210.941 379.814 212.479 380.834C214.238 382 217.011 382 222.557 382H276.719C282.184 382 284.917 382 286.666 380.852C288.196 379.847 289.272 378.283 289.663 376.495C290.11 374.451 289.133 371.899 287.178 366.795L250.5 271Z\" fill=\"url(#paint9_linear_81_297)\"/>");
    svg.push_str("</g>");

    // 
    svg.push_str(&format!(
      "<text x=\"39\" y=\"75%\" text-anchor=\"start\" dominant-baseline=\"middle\" font-family=\"Inter, system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif\" font-size=\"24\" fill=\"#f3f4f6\" font-weight=\"600\" letter-spacing=\"2px\" filter=\"url(#textShadow)\">{}.btc</text>",
      domain_name
    ));

    svg.push_str("<line x1=\"0\" y1=\"220\" x2=\"270\" y2=\"220\" stroke=\"#f3f4f6\" stroke-width=\"4\" stroke-opacity=\"0.2\"/>");

    svg.push_str("</svg>");

    Ok(svg)
  }
} 
