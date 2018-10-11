ios:
	cargo build --target aarch64-apple-ios

android:
	cargo build --target aarch64-linux-android 
	cargo build --target armv7-linux-androideabi 
	cargo build --target i686-linux-android 
	cp target/aarch64-linux-android/debug/libgpuimage.so ~/XCamera_Andriod/app/src/main/jniLibs/arm64-v8a
	cp target/armv7-linux-androideabi/debug/libgpuimage.so ~/XCamera_Andriod/app/src/main/jniLibs/armeabi-v7a
	cp target/i686-linux-android/debug/libgpuimage.so ~/XCamera_Andriod/app/src/main/jniLibs/x86

clean:
	cargo clean
