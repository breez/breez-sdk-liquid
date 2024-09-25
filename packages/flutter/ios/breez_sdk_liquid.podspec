version = '0.3.4' # generated; do not edit
# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint breez_sdk.podspec` to validate before publishing.
Pod::Spec.new do |s|
  s.name                   = 'BreezSDKLiquid'
  s.version                = "#{version}"
  s.license                = { :type => "MIT" }
  s.summary                = "Swift bindings to the Breez Liquid SDK"
  s.homepage               = "https://breez.technology"
  s.authors                = { "Breez" => "contact@breez.technology" }
  s.documentation_url      = "https://sdk-doc.breez.technology"
  s.ios.deployment_target  = "13.0"
  s.source                 = { :path => '.' }
  s.source_files           = [
    'bindings-swift/Sources/BreezSDKLiquid/*.swift', 
    'bindings-swift/Sources/BreezSDKLiquid/**/*.swift'
  ]
  s.platform               = :ios, '13.0'
  s.static_framework       = true
  s.vendored_frameworks    = "bindings-swift/breez_sdk_liquidFFI.xcframework"

  # Flutter.framework does not contain a i386 slice.
  s.pod_target_xcconfig    = {'STRIP_STYLE' => 'non-global', 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  s.swift_version          = '5.0'
end
