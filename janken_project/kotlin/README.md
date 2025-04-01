# Kotlinの実行手順
jarファイルを生成して、javaで実行する
```zsh
kotlinc Janken.kt -include-runtime -d Janken.jar
java -jar Janken.jar 
```