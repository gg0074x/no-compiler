import XCTest
import SwiftTreeSitter
import TreeSitterNc

final class TreeSitterNcTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_nc())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Nc grammar")
    }
}
