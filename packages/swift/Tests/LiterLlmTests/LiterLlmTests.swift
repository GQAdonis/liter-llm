import XCTest

@testable import LiterLlm

final class LiterLlmTests: XCTestCase {
  func testSystemMessageRoundTripsThroughJSON() throws {
    let message = SystemMessage(content: .text(field0: "be concise"), name: "system")
    let data = try JSONEncoder().encode(message)
    let decoded = try JSONDecoder().decode(SystemMessage.self, from: data)
    XCTAssertEqual(decoded, message)
  }
}
