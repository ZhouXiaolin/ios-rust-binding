ios:
	cargo build --target aarch64-apple-ios

android:
	cargo build --target aarch64-linux-android --release
	cargo build --target armv7-linux-androideabi --release
	cargo build --target i686-linux-android --release
	cp target/aarch64-linux-android/release/libgpuimage.so ~/XCamera_Andriod/app/libs/arm64-v8a
	cp target/armv7-linux-androideabi/release/libgpuimage.so ~/XCamera_Andriod/app/libs/armeabi-v7a

clean:
	cargo clean
