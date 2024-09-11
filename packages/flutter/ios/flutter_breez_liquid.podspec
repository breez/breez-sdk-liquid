version = '0.3.1-dev1' # generated; do not edit
tag_name = "v#{version}"
release_tag_name = "breez_liquid-#{tag_name}"

# We cannot distribute the XCFramework alongside the library directly,
# so we have to fetch the correct version here.
framework_name = 'breez_sdk_liquid.xcframework'
remote_zip_name = "#{framework_name}.zip"
local_zip_name = "#{release_tag_name}.zip"

# To learn more about a Podspec see http://guides.cocoapods.org/syntax/podspec.html.
# Run `pod lib lint flutter_breez_liquid.podspec` to validate before publishing.
Pod::Spec.new do |spec|
  spec.name          = 'flutter_breez_liquid'
  spec.version       = "#{version}"
  spec.license       = { :file => '../LICENSE', :type => 'MIT License' }
  spec.homepage      = 'https://breez.technology'
  spec.authors       = { 'Breez' => 'contact@breez.technology' }
  spec.summary       = 'iOS/macOS Flutter bindings for Breez Liquid'

  spec.source              = { :path => '.' }
  spec.source_files        = 'Classes/**/*'
  spec.public_header_files = 'Classes/**/*.h'
  spec.vendored_frameworks = "Frameworks/#{framework_name}"

  spec.prepare_command = <<-CMD
    cd Frameworks
    rm -rf #{framework_name}
    unzip #{local_zip_name}
    cd -
  CMD
  
  spec.ios.deployment_target = '12.0'
  spec.osx.deployment_target = '10.11'

  spec.dependency 'Flutter'
  spec.static_framework = true

  # Flutter.framework does not contain a i386 slice.
  spec.pod_target_xcconfig = {'STRIP_STYLE' => 'non-global', 'DEFINES_MODULE' => 'YES', 'EXCLUDED_ARCHS[sdk=iphonesimulator*]' => 'i386' }
  spec.swift_version = '5.0'
end
