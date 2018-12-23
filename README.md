# pero
A command to list project directories (like du command for projects)

```
% cargo install

% pero .
./solr-in-action: Maven (0)
./unfiltered-scalate.g8: sbt (0)
./unfiltered-scalate.g8/src/main/g8: sbt (0)
./scraper/project: sbt (1,507,328)
./dust: Cargo (129,257,472)
./kuickcheck: Gradle (0)
./android-floating-action-button/library: Gradle (450,560)
./android-floating-action-button: Gradle (155,648)
./android-floating-action-button/sample: Gradle (782,336)
./react-redux-realworld-example-app: npm (171,016,192)

...

Statistics
==========
npm: 16, 3,317,547,008
Gradle: 33, 766,279,680
Maven: 46, 0
sbt: 40, 480,518,144
Cargo: 42, 300,384,256
==========
Total: 177, 4,864,729,088
```
