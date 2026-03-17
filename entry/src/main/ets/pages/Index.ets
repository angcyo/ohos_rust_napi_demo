import * as lib from 'librust_hello.so';

@Entry
@Component
struct Index {
  @State message: string = 'Hello World';

  build() {
    RelativeContainer() {
      Text(`ArtTS使用Rust编译产物.so示例\n` +
        `testBool->${lib.testBool(false)}\n` +
        `testInt->${lib.testInt(10)}\n` +
        `testFloat->${lib.testDouble(10.0)}\n` +
        `testString->${lib.testString('hello')}\n` +
        `testArray->${lib.testBytes([1, 2, 3])}\n` +
        `testArray->${lib.testBuffer(new ArrayBuffer(10))}\n`
      )
        .id('HelloWorld')
        .fontSize($r('app.float.page_text_font_size'))
        .fontWeight(FontWeight.Bold)
        .alignRules({
          center: { anchor: '__container__', align: VerticalAlign.Center },
          middle: { anchor: '__container__', align: HorizontalAlign.Center }
        })
        .onClick(() => {
          this.message = 'Welcome';
        })
    }
    .height('100%')
    .width('100%')
  }
}