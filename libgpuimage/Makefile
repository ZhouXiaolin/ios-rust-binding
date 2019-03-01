ios:
	cargo build --target aarch64-apple-ios --release
	cp target/aarch64-apple-ios/release/libgpuimage.a ~/Documents/RustGPUImage/RustOpenGLTest/libGPUImage
	cp ~/Documents/RustGPUImage/libgpuimage/src/gpuimage.h ~/Documents/RustGPUImage/RustOpenGLTest/libGPUImage

android:
	cargo build --target aarch64-linux-android --release
	cargo build --target armv7-linux-androideabi --release
	# cargo build --target i686-linux-android --release
	# cp target/aarch64-linux-android/release/libgpuimage.so ~/XCamera_Andriod/app/src/main/jniLibs/arm64-v8a
	cp target/armv7-linux-androideabi/release/libgpuimage.so ~/XCamera_Andriod/app/src/main/jniLibs/armeabi-v7a
	# cp target/i686-linux-android/release/libgpuimage.so ~/XCamera_Andriod/app/src/main/jniLibs/x86
	# cp target/aarch64-linux-android/release/libgpuimage.so ~/XCameraAndroidLibrary/app/src/main/jniLibs/arm64-v8a
	# cp target/armv7-linux-androideabi/release/libgpuimage.so ~/XCameraAndroidLibrary/app/src/main/jniLibs/armeabi-v7a
clean:
	cargo clean
