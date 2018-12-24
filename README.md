# pero
A command to list project directories (like du command for projects)

```
% cargo install

% pero .
./unfiltered-scalate.g8: sbt (0)
./unfiltered-scalate.g8/src/main/g8: sbt (0)
./scraper/project: sbt (1,507,328)
./dust: Cargo (129,257,472)
./kuickcheck: Gradle (0)
./android-floating-action-button/library: Gradle (450,560)
./android-floating-action-button: Gradle (155,648)
./android-floating-action-button/sample: Gradle (782,336)
...
./react-router/packages/react-router-native/android/app: Gradle (0)
./react-router/packages/react-router-native/android: Gradle (0)
./android-ndk-r10d/samples/hello-jni: Gradle (0)

Statistics
==========
Cargo: 41 projects, 238,518,272 bytes
npm: 3 projects, 388,665,344 bytes
sbt: 36 projects, 287,334,400 bytes
Gradle: 20 projects, 1,388,544 bytes
Maven: 42 projects, 0 bytes
----------
Total: 142 projects, 915,906,560 bytes
```

## Supported project types
- Maven
- Gradle
- sbt
- Cargo
- npm
