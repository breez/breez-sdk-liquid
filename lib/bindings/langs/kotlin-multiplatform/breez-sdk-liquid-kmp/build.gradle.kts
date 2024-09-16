plugins {
    kotlin("multiplatform")
    id("com.android.library")
    id("maven-publish")
}

apply(plugin = "kotlinx-atomicfu")

kotlin {
    // Enable the default target hierarchy
    applyDefaultHierarchyTemplate()

    androidTarget {
        compilations.all {
            kotlinOptions {
                jvmTarget = JavaVersion.VERSION_17.majorVersion
            }
        }

        publishLibraryVariants("release")
    }

    jvm {
        compilations.all {
            kotlinOptions.jvmTarget = JavaVersion.VERSION_17.majorVersion
        }
    }

    listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    ).forEach {
        val platform = when (it.targetName) {
            "iosSimulatorArm64" -> "ios_simulator_arm64"
            "iosArm64" -> "ios_arm64"
            "iosX64" -> "ios_x64"
            else -> error("Unsupported target $name")
        }

        it.compilations["main"].cinterops {
            create("breezCInterop") {
                defFile(project.file("src/nativeInterop/cinterop/breez.def"))
                includeDirs(project.file("src/nativeInterop/cinterop/headers/breez_sdk_liquid"), project.file("src/lib/$platform"))
            }
        }
    }

    sourceSets {
        all {
            languageSettings.apply {
                optIn("kotlinx.cinterop.ExperimentalForeignApi")
            }
        }

        val commonMain by getting {
            dependencies {
                implementation("com.squareup.okio:okio:3.6.0")
                implementation("org.jetbrains.kotlinx:kotlinx-datetime:0.5.0")
                implementation("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.8.0")
            }
        }

        val jvmMain by getting {
            dependsOn(commonMain)
            dependencies {
                implementation("net.java.dev.jna:jna:5.13.0")
            }
        }

        val androidMain by getting {
            dependsOn(commonMain)
            dependencies {
                implementation("net.java.dev.jna:jna:5.13.0@aar")
                implementation("org.jetbrains.kotlinx:atomicfu:0.23.1")
            }
        }
    }
}

android {
    namespace = "technology.breez.liquid"
    compileSdk = 33

    defaultConfig {
        minSdk = 21
        consumerProguardFiles("consumer-rules.pro")
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
}

val libraryVersion: String by project

group = "technology.breez.liquid"
version = libraryVersion

publishing {
    repositories {
        maven {
            name = "breezReposilite"
            url = uri("https://mvn.breez.technology/releases")
            credentials(PasswordCredentials::class)
            authentication {
                create<BasicAuthentication>("basic")
            }
        }
    }

    publications {
        this.forEach {
            (it as MavenPublication).apply {
                pom {
                    name.set("breez-sdk-liquid-kmp")
                    description.set("The Breez Liquid SDK enables mobile developers to integrate Liquid swaps into their apps with a very shallow learning curve.")
                    url.set("https://breez.technology")
                    licenses {
                        license {
                            name.set("MIT")
                            url.set("https://github.com/breez/breez-sdk-liquid/blob/main/LICENSE")
                        }
                    }
                    scm {
                        connection.set("scm:git:github.com/breez/breez-sdk-liquid.git")
                        developerConnection.set("scm:git:ssh://github.com/breez/breez-sdk-liquid.git")
                        url.set("https://github.com/breez/breez-sdk-liquid")
                    }
                }
            }
        }
    }
}