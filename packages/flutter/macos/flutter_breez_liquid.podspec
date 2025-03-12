version = '0.7.3' # generated; do not edit

# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint flutter_breez_liquid.podspec` to validate before publishing.
Pod::Spec.new do |spec|
  spec.name          = 'flutter_breez_liquid'
  spec.version       = "#{version}"
  spec.license       = { :file => '../LICENSE', :type => 'MIT License' }
  spec.homepage      = 'https://breez.technology'
  spec.authors       = { 'Breez' => 'contact@breez.technology' }
  spec.summary       = 'iOS/macOS Flutter bindings for Breez Liquid SDK'
  spec.documentation_url      = "https://sdk-doc.breez.technology"

  spec.source              = { :path => '.' }
  spec.source_files        = 'Classes/**/*'
  spec.public_header_files = 'Classes/**/*.h'
  spec.on_demand_resources = { 
    'BreezSDKLiquid' => [
      'Sources/BreezSDKLiquid/*.swift', 
      'Sources/BreezSDKLiquid/**/*.swift'
    ]
  }

  spec.vendored_frameworks = "Frameworks/breez_sdk_liquidFFI.xcframework"

  spec.ios.deployment_target = '13.0'
  spec.osx.deployment_target = '15.0'

  spec.dependency 'Flutter'
  # Flutter.framework does not contain a i386 slice.
  spec.pod_target_xcconfig = {
    'STRIP_STYLE' => 'non-global',
    'DEFINES_MODULE' => 'YES',
    'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386',
  }
  spec.swift_version = '5.0'
end
