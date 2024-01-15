import uniffi.external_foreign_trait.*;
import uniffi.shared.*;

val tester = object : Tester {
    override fun test(): String {
        return "Hello from Kotlin!"
    }
}

assert(doTest(tester) == "Hello from Kotlin!")
