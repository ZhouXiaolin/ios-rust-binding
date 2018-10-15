//
//  main.swift
//  Scheme
//
//  Created by 周晓林 on 2018/10/11.
//  Copyright © 2018 Solaren. All rights reserved.
//

import Foundation

enum Token {
    case OpenParen
    case CloseParen
    case Quote
    case Quasiquote
    case Unquote
    case Identifier(String)
    case Integer(Int64)
    case Boolean(Bool)
    case String(String)
}

struct SyntaxError : Error {
    var message: String
    var line: Int
    var column: Int
}

enum Result<T,E> {
    case Ok(T)
    case Err(E)
}


struct Lexer{
    var chars : String.Iterator
    var _current : Character?
    var tokens: [Token]
    var line : UInt32
    var column : UInt32
}

extension Lexer {
    func tokenize(s : String) -> Result<[Token], SyntaxError> {
        let lexer = Lexer.init(chars: s.makeIterator(), _current: .none, tokens: [], line: 1, column: 0)
        lexer.run()
        return Result.Ok(lexer.tokens)
    }

    func current() -> Character? {
        return self._current
    }

    mutating func advance() {
        if let _ = self.current() {
            self.line += 1
            self.column = 1
        }else {
            self.column += 1
        }
        self._current = self.chars.next()
    }
    mutating func run() {
        self.advance()
        while true {
            switch self.current() {
            case .some(let c):
            
                if c == " " {
                    self.advance()
                }else if c == ";" {
                    self.advance()
                    while true {
                        if let c = self.current() {
                            if c == "\n" {
                                self.advance()
                                break
                            }else{
                                self.advance()
                            }
                        }else{
                            break
                        }
                    }
                }else if c == "(" {
                    self.tokens.append(Token.OpenParen)
                    self.advance()
                }else if c == ")" {
                    self.tokens.append(Token.CloseParen)
                    self.advance()
                }else if c == "'" {
                    self.tokens.append(Token.Quote)
                    self.advance()
                }else if c == "`" {
                    self.tokens.append(Token.Quasiquote)
                    self.advance()
                }else if c == "," {
                    self.tokens.append(Token.Unquote)
                    self.advance()
                }else if c == "+" || c == "-" {
                    switch self.peek() {
                    }
                }
            
            case .none:
                <#code#>
            
            }
        }
    }
    
    func peek() -> Character? {
        var it = self.chars.makeIterator()
        if let c = it.next() {
            return c
        }else{
            return .none
        }
    }
}


