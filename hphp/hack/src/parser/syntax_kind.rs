/**
 * Copyright (c) 2016, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree. An additional
 * directory.
 *
 **
 *
 * THIS FILE IS @generated; DO NOT EDIT IT
 * To regenerate this file, run
 *
 *   buck run //hphp/hack/src:generate_full_fidelity
 *
 **
 *
 */

use ocamlrep_derive::{FromOcamlRep, ToOcamlRep};

use crate::token_kind::TokenKind;

#[derive(Debug, Copy, Clone, FromOcamlRep, ToOcamlRep, PartialEq)]
pub enum SyntaxKind {
    Missing,
    Token(TokenKind),
    SyntaxList,
    EndOfFile,
    Script,
    QualifiedName,
    SimpleTypeSpecifier,
    LiteralExpression,
    PrefixedStringExpression,
    PrefixedCodeExpression,
    VariableExpression,
    PipeVariableExpression,
    FileAttributeSpecification,
    EnumDeclaration,
    EnumUse,
    Enumerator,
    EnumClassDeclaration,
    EnumClassEnumerator,
    AliasDeclaration,
    ContextAliasDeclaration,
    PropertyDeclaration,
    PropertyDeclarator,
    NamespaceDeclaration,
    NamespaceDeclarationHeader,
    NamespaceBody,
    NamespaceEmptyBody,
    NamespaceUseDeclaration,
    NamespaceGroupUseDeclaration,
    NamespaceUseClause,
    FunctionDeclaration,
    FunctionDeclarationHeader,
    Contexts,
    WhereClause,
    WhereConstraint,
    MethodishDeclaration,
    MethodishTraitResolution,
    ClassishDeclaration,
    ClassishBody,
    TraitUse,
    RequireClause,
    ConstDeclaration,
    ConstantDeclarator,
    TypeConstDeclaration,
    ContextConstDeclaration,
    DecoratedExpression,
    ParameterDeclaration,
    VariadicParameter,
    OldAttributeSpecification,
    AttributeSpecification,
    Attribute,
    InclusionExpression,
    InclusionDirective,
    CompoundStatement,
    ExpressionStatement,
    MarkupSection,
    MarkupSuffix,
    UnsetStatement,
    UsingStatementBlockScoped,
    UsingStatementFunctionScoped,
    WhileStatement,
    IfStatement,
    ElseClause,
    TryStatement,
    CatchClause,
    FinallyClause,
    DoStatement,
    ForStatement,
    ForeachStatement,
    SwitchStatement,
    SwitchSection,
    SwitchFallthrough,
    CaseLabel,
    DefaultLabel,
    ReturnStatement,
    YieldBreakStatement,
    ThrowStatement,
    BreakStatement,
    ContinueStatement,
    EchoStatement,
    ConcurrentStatement,
    SimpleInitializer,
    AnonymousClass,
    AnonymousFunction,
    AnonymousFunctionUseClause,
    LambdaExpression,
    LambdaSignature,
    CastExpression,
    ScopeResolutionExpression,
    MemberSelectionExpression,
    SafeMemberSelectionExpression,
    EmbeddedMemberSelectionExpression,
    YieldExpression,
    PrefixUnaryExpression,
    PostfixUnaryExpression,
    BinaryExpression,
    IsExpression,
    AsExpression,
    NullableAsExpression,
    UpcastExpression,
    ConditionalExpression,
    EvalExpression,
    IssetExpression,
    FunctionCallExpression,
    FunctionPointerExpression,
    ParenthesizedExpression,
    BracedExpression,
    ETSpliceExpression,
    EmbeddedBracedExpression,
    ListExpression,
    CollectionLiteralExpression,
    ObjectCreationExpression,
    ConstructorCall,
    DarrayIntrinsicExpression,
    DictionaryIntrinsicExpression,
    KeysetIntrinsicExpression,
    VarrayIntrinsicExpression,
    VectorIntrinsicExpression,
    ElementInitializer,
    SubscriptExpression,
    EmbeddedSubscriptExpression,
    AwaitableCreationExpression,
    XHPChildrenDeclaration,
    XHPChildrenParenthesizedList,
    XHPCategoryDeclaration,
    XHPEnumType,
    XHPLateinit,
    XHPRequired,
    XHPClassAttributeDeclaration,
    XHPClassAttribute,
    XHPSimpleClassAttribute,
    XHPSimpleAttribute,
    XHPSpreadAttribute,
    XHPOpen,
    XHPExpression,
    XHPClose,
    TypeConstant,
    VectorTypeSpecifier,
    KeysetTypeSpecifier,
    TupleTypeExplicitSpecifier,
    VarrayTypeSpecifier,
    FunctionCtxTypeSpecifier,
    TypeParameter,
    TypeConstraint,
    ContextConstraint,
    DarrayTypeSpecifier,
    DictionaryTypeSpecifier,
    ClosureTypeSpecifier,
    ClosureParameterTypeSpecifier,
    ClassnameTypeSpecifier,
    FieldSpecifier,
    FieldInitializer,
    ShapeTypeSpecifier,
    ShapeExpression,
    TupleExpression,
    GenericTypeSpecifier,
    NullableTypeSpecifier,
    LikeTypeSpecifier,
    SoftTypeSpecifier,
    AttributizedSpecifier,
    ReifiedTypeArgument,
    TypeArguments,
    TypeParameters,
    TupleTypeSpecifier,
    UnionTypeSpecifier,
    IntersectionTypeSpecifier,
    ErrorSyntax,
    ListItem,
    EnumClassLabelExpression,
    ModuleDeclaration,
    ModuleMembershipDeclaration,

}

impl SyntaxKind {
    pub fn to_string(&self) -> &str {
        match self {
            SyntaxKind::SyntaxList => "list",
            SyntaxKind::Missing => "missing",
            SyntaxKind::Token(_) => "token",
            SyntaxKind::EndOfFile                         => "end_of_file",
            SyntaxKind::Script                            => "script",
            SyntaxKind::QualifiedName                     => "qualified_name",
            SyntaxKind::SimpleTypeSpecifier               => "simple_type_specifier",
            SyntaxKind::LiteralExpression                 => "literal",
            SyntaxKind::PrefixedStringExpression          => "prefixed_string",
            SyntaxKind::PrefixedCodeExpression            => "prefixed_code",
            SyntaxKind::VariableExpression                => "variable",
            SyntaxKind::PipeVariableExpression            => "pipe_variable",
            SyntaxKind::FileAttributeSpecification        => "file_attribute_specification",
            SyntaxKind::EnumDeclaration                   => "enum_declaration",
            SyntaxKind::EnumUse                           => "enum_use",
            SyntaxKind::Enumerator                        => "enumerator",
            SyntaxKind::EnumClassDeclaration              => "enum_class_declaration",
            SyntaxKind::EnumClassEnumerator               => "enum_class_enumerator",
            SyntaxKind::AliasDeclaration                  => "alias_declaration",
            SyntaxKind::ContextAliasDeclaration           => "context_alias_declaration",
            SyntaxKind::PropertyDeclaration               => "property_declaration",
            SyntaxKind::PropertyDeclarator                => "property_declarator",
            SyntaxKind::NamespaceDeclaration              => "namespace_declaration",
            SyntaxKind::NamespaceDeclarationHeader        => "namespace_declaration_header",
            SyntaxKind::NamespaceBody                     => "namespace_body",
            SyntaxKind::NamespaceEmptyBody                => "namespace_empty_body",
            SyntaxKind::NamespaceUseDeclaration           => "namespace_use_declaration",
            SyntaxKind::NamespaceGroupUseDeclaration      => "namespace_group_use_declaration",
            SyntaxKind::NamespaceUseClause                => "namespace_use_clause",
            SyntaxKind::FunctionDeclaration               => "function_declaration",
            SyntaxKind::FunctionDeclarationHeader         => "function_declaration_header",
            SyntaxKind::Contexts                          => "contexts",
            SyntaxKind::WhereClause                       => "where_clause",
            SyntaxKind::WhereConstraint                   => "where_constraint",
            SyntaxKind::MethodishDeclaration              => "methodish_declaration",
            SyntaxKind::MethodishTraitResolution          => "methodish_trait_resolution",
            SyntaxKind::ClassishDeclaration               => "classish_declaration",
            SyntaxKind::ClassishBody                      => "classish_body",
            SyntaxKind::TraitUse                          => "trait_use",
            SyntaxKind::RequireClause                     => "require_clause",
            SyntaxKind::ConstDeclaration                  => "const_declaration",
            SyntaxKind::ConstantDeclarator                => "constant_declarator",
            SyntaxKind::TypeConstDeclaration              => "type_const_declaration",
            SyntaxKind::ContextConstDeclaration           => "context_const_declaration",
            SyntaxKind::DecoratedExpression               => "decorated_expression",
            SyntaxKind::ParameterDeclaration              => "parameter_declaration",
            SyntaxKind::VariadicParameter                 => "variadic_parameter",
            SyntaxKind::OldAttributeSpecification         => "old_attribute_specification",
            SyntaxKind::AttributeSpecification            => "attribute_specification",
            SyntaxKind::Attribute                         => "attribute",
            SyntaxKind::InclusionExpression               => "inclusion_expression",
            SyntaxKind::InclusionDirective                => "inclusion_directive",
            SyntaxKind::CompoundStatement                 => "compound_statement",
            SyntaxKind::ExpressionStatement               => "expression_statement",
            SyntaxKind::MarkupSection                     => "markup_section",
            SyntaxKind::MarkupSuffix                      => "markup_suffix",
            SyntaxKind::UnsetStatement                    => "unset_statement",
            SyntaxKind::UsingStatementBlockScoped         => "using_statement_block_scoped",
            SyntaxKind::UsingStatementFunctionScoped      => "using_statement_function_scoped",
            SyntaxKind::WhileStatement                    => "while_statement",
            SyntaxKind::IfStatement                       => "if_statement",
            SyntaxKind::ElseClause                        => "else_clause",
            SyntaxKind::TryStatement                      => "try_statement",
            SyntaxKind::CatchClause                       => "catch_clause",
            SyntaxKind::FinallyClause                     => "finally_clause",
            SyntaxKind::DoStatement                       => "do_statement",
            SyntaxKind::ForStatement                      => "for_statement",
            SyntaxKind::ForeachStatement                  => "foreach_statement",
            SyntaxKind::SwitchStatement                   => "switch_statement",
            SyntaxKind::SwitchSection                     => "switch_section",
            SyntaxKind::SwitchFallthrough                 => "switch_fallthrough",
            SyntaxKind::CaseLabel                         => "case_label",
            SyntaxKind::DefaultLabel                      => "default_label",
            SyntaxKind::ReturnStatement                   => "return_statement",
            SyntaxKind::YieldBreakStatement               => "yield_break_statement",
            SyntaxKind::ThrowStatement                    => "throw_statement",
            SyntaxKind::BreakStatement                    => "break_statement",
            SyntaxKind::ContinueStatement                 => "continue_statement",
            SyntaxKind::EchoStatement                     => "echo_statement",
            SyntaxKind::ConcurrentStatement               => "concurrent_statement",
            SyntaxKind::SimpleInitializer                 => "simple_initializer",
            SyntaxKind::AnonymousClass                    => "anonymous_class",
            SyntaxKind::AnonymousFunction                 => "anonymous_function",
            SyntaxKind::AnonymousFunctionUseClause        => "anonymous_function_use_clause",
            SyntaxKind::LambdaExpression                  => "lambda_expression",
            SyntaxKind::LambdaSignature                   => "lambda_signature",
            SyntaxKind::CastExpression                    => "cast_expression",
            SyntaxKind::ScopeResolutionExpression         => "scope_resolution_expression",
            SyntaxKind::MemberSelectionExpression         => "member_selection_expression",
            SyntaxKind::SafeMemberSelectionExpression     => "safe_member_selection_expression",
            SyntaxKind::EmbeddedMemberSelectionExpression => "embedded_member_selection_expression",
            SyntaxKind::YieldExpression                   => "yield_expression",
            SyntaxKind::PrefixUnaryExpression             => "prefix_unary_expression",
            SyntaxKind::PostfixUnaryExpression            => "postfix_unary_expression",
            SyntaxKind::BinaryExpression                  => "binary_expression",
            SyntaxKind::IsExpression                      => "is_expression",
            SyntaxKind::AsExpression                      => "as_expression",
            SyntaxKind::NullableAsExpression              => "nullable_as_expression",
            SyntaxKind::UpcastExpression                  => "upcast_expression",
            SyntaxKind::ConditionalExpression             => "conditional_expression",
            SyntaxKind::EvalExpression                    => "eval_expression",
            SyntaxKind::IssetExpression                   => "isset_expression",
            SyntaxKind::FunctionCallExpression            => "function_call_expression",
            SyntaxKind::FunctionPointerExpression         => "function_pointer_expression",
            SyntaxKind::ParenthesizedExpression           => "parenthesized_expression",
            SyntaxKind::BracedExpression                  => "braced_expression",
            SyntaxKind::ETSpliceExpression                => "et_splice_expression",
            SyntaxKind::EmbeddedBracedExpression          => "embedded_braced_expression",
            SyntaxKind::ListExpression                    => "list_expression",
            SyntaxKind::CollectionLiteralExpression       => "collection_literal_expression",
            SyntaxKind::ObjectCreationExpression          => "object_creation_expression",
            SyntaxKind::ConstructorCall                   => "constructor_call",
            SyntaxKind::DarrayIntrinsicExpression         => "darray_intrinsic_expression",
            SyntaxKind::DictionaryIntrinsicExpression     => "dictionary_intrinsic_expression",
            SyntaxKind::KeysetIntrinsicExpression         => "keyset_intrinsic_expression",
            SyntaxKind::VarrayIntrinsicExpression         => "varray_intrinsic_expression",
            SyntaxKind::VectorIntrinsicExpression         => "vector_intrinsic_expression",
            SyntaxKind::ElementInitializer                => "element_initializer",
            SyntaxKind::SubscriptExpression               => "subscript_expression",
            SyntaxKind::EmbeddedSubscriptExpression       => "embedded_subscript_expression",
            SyntaxKind::AwaitableCreationExpression       => "awaitable_creation_expression",
            SyntaxKind::XHPChildrenDeclaration            => "xhp_children_declaration",
            SyntaxKind::XHPChildrenParenthesizedList      => "xhp_children_parenthesized_list",
            SyntaxKind::XHPCategoryDeclaration            => "xhp_category_declaration",
            SyntaxKind::XHPEnumType                       => "xhp_enum_type",
            SyntaxKind::XHPLateinit                       => "xhp_lateinit",
            SyntaxKind::XHPRequired                       => "xhp_required",
            SyntaxKind::XHPClassAttributeDeclaration      => "xhp_class_attribute_declaration",
            SyntaxKind::XHPClassAttribute                 => "xhp_class_attribute",
            SyntaxKind::XHPSimpleClassAttribute           => "xhp_simple_class_attribute",
            SyntaxKind::XHPSimpleAttribute                => "xhp_simple_attribute",
            SyntaxKind::XHPSpreadAttribute                => "xhp_spread_attribute",
            SyntaxKind::XHPOpen                           => "xhp_open",
            SyntaxKind::XHPExpression                     => "xhp_expression",
            SyntaxKind::XHPClose                          => "xhp_close",
            SyntaxKind::TypeConstant                      => "type_constant",
            SyntaxKind::VectorTypeSpecifier               => "vector_type_specifier",
            SyntaxKind::KeysetTypeSpecifier               => "keyset_type_specifier",
            SyntaxKind::TupleTypeExplicitSpecifier        => "tuple_type_explicit_specifier",
            SyntaxKind::VarrayTypeSpecifier               => "varray_type_specifier",
            SyntaxKind::FunctionCtxTypeSpecifier          => "function_ctx_type_specifier",
            SyntaxKind::TypeParameter                     => "type_parameter",
            SyntaxKind::TypeConstraint                    => "type_constraint",
            SyntaxKind::ContextConstraint                 => "context_constraint",
            SyntaxKind::DarrayTypeSpecifier               => "darray_type_specifier",
            SyntaxKind::DictionaryTypeSpecifier           => "dictionary_type_specifier",
            SyntaxKind::ClosureTypeSpecifier              => "closure_type_specifier",
            SyntaxKind::ClosureParameterTypeSpecifier     => "closure_parameter_type_specifier",
            SyntaxKind::ClassnameTypeSpecifier            => "classname_type_specifier",
            SyntaxKind::FieldSpecifier                    => "field_specifier",
            SyntaxKind::FieldInitializer                  => "field_initializer",
            SyntaxKind::ShapeTypeSpecifier                => "shape_type_specifier",
            SyntaxKind::ShapeExpression                   => "shape_expression",
            SyntaxKind::TupleExpression                   => "tuple_expression",
            SyntaxKind::GenericTypeSpecifier              => "generic_type_specifier",
            SyntaxKind::NullableTypeSpecifier             => "nullable_type_specifier",
            SyntaxKind::LikeTypeSpecifier                 => "like_type_specifier",
            SyntaxKind::SoftTypeSpecifier                 => "soft_type_specifier",
            SyntaxKind::AttributizedSpecifier             => "attributized_specifier",
            SyntaxKind::ReifiedTypeArgument               => "reified_type_argument",
            SyntaxKind::TypeArguments                     => "type_arguments",
            SyntaxKind::TypeParameters                    => "type_parameters",
            SyntaxKind::TupleTypeSpecifier                => "tuple_type_specifier",
            SyntaxKind::UnionTypeSpecifier                => "union_type_specifier",
            SyntaxKind::IntersectionTypeSpecifier         => "intersection_type_specifier",
            SyntaxKind::ErrorSyntax                       => "error",
            SyntaxKind::ListItem                          => "list_item",
            SyntaxKind::EnumClassLabelExpression          => "enum_class_label",
            SyntaxKind::ModuleDeclaration                 => "module_declaration",
            SyntaxKind::ModuleMembershipDeclaration       => "module_membership_declaration",
        }
    }

    pub fn ocaml_tag(self) -> u8 {
        match self {
            SyntaxKind::Missing => 0,
            SyntaxKind::Token(_) => 0,
            SyntaxKind::SyntaxList => 1,
            SyntaxKind::EndOfFile => 2,
            SyntaxKind::Script => 3,
            SyntaxKind::QualifiedName => 4,
            SyntaxKind::SimpleTypeSpecifier => 5,
            SyntaxKind::LiteralExpression => 6,
            SyntaxKind::PrefixedStringExpression => 7,
            SyntaxKind::PrefixedCodeExpression => 8,
            SyntaxKind::VariableExpression => 9,
            SyntaxKind::PipeVariableExpression => 10,
            SyntaxKind::FileAttributeSpecification => 11,
            SyntaxKind::EnumDeclaration => 12,
            SyntaxKind::EnumUse => 13,
            SyntaxKind::Enumerator => 14,
            SyntaxKind::EnumClassDeclaration => 15,
            SyntaxKind::EnumClassEnumerator => 16,
            SyntaxKind::AliasDeclaration => 17,
            SyntaxKind::ContextAliasDeclaration => 18,
            SyntaxKind::PropertyDeclaration => 19,
            SyntaxKind::PropertyDeclarator => 20,
            SyntaxKind::NamespaceDeclaration => 21,
            SyntaxKind::NamespaceDeclarationHeader => 22,
            SyntaxKind::NamespaceBody => 23,
            SyntaxKind::NamespaceEmptyBody => 24,
            SyntaxKind::NamespaceUseDeclaration => 25,
            SyntaxKind::NamespaceGroupUseDeclaration => 26,
            SyntaxKind::NamespaceUseClause => 27,
            SyntaxKind::FunctionDeclaration => 28,
            SyntaxKind::FunctionDeclarationHeader => 29,
            SyntaxKind::Contexts => 30,
            SyntaxKind::WhereClause => 31,
            SyntaxKind::WhereConstraint => 32,
            SyntaxKind::MethodishDeclaration => 33,
            SyntaxKind::MethodishTraitResolution => 34,
            SyntaxKind::ClassishDeclaration => 35,
            SyntaxKind::ClassishBody => 36,
            SyntaxKind::TraitUse => 37,
            SyntaxKind::RequireClause => 38,
            SyntaxKind::ConstDeclaration => 39,
            SyntaxKind::ConstantDeclarator => 40,
            SyntaxKind::TypeConstDeclaration => 41,
            SyntaxKind::ContextConstDeclaration => 42,
            SyntaxKind::DecoratedExpression => 43,
            SyntaxKind::ParameterDeclaration => 44,
            SyntaxKind::VariadicParameter => 45,
            SyntaxKind::OldAttributeSpecification => 46,
            SyntaxKind::AttributeSpecification => 47,
            SyntaxKind::Attribute => 48,
            SyntaxKind::InclusionExpression => 49,
            SyntaxKind::InclusionDirective => 50,
            SyntaxKind::CompoundStatement => 51,
            SyntaxKind::ExpressionStatement => 52,
            SyntaxKind::MarkupSection => 53,
            SyntaxKind::MarkupSuffix => 54,
            SyntaxKind::UnsetStatement => 55,
            SyntaxKind::UsingStatementBlockScoped => 56,
            SyntaxKind::UsingStatementFunctionScoped => 57,
            SyntaxKind::WhileStatement => 58,
            SyntaxKind::IfStatement => 59,
            SyntaxKind::ElseClause => 60,
            SyntaxKind::TryStatement => 61,
            SyntaxKind::CatchClause => 62,
            SyntaxKind::FinallyClause => 63,
            SyntaxKind::DoStatement => 64,
            SyntaxKind::ForStatement => 65,
            SyntaxKind::ForeachStatement => 66,
            SyntaxKind::SwitchStatement => 67,
            SyntaxKind::SwitchSection => 68,
            SyntaxKind::SwitchFallthrough => 69,
            SyntaxKind::CaseLabel => 70,
            SyntaxKind::DefaultLabel => 71,
            SyntaxKind::ReturnStatement => 72,
            SyntaxKind::YieldBreakStatement => 73,
            SyntaxKind::ThrowStatement => 74,
            SyntaxKind::BreakStatement => 75,
            SyntaxKind::ContinueStatement => 76,
            SyntaxKind::EchoStatement => 77,
            SyntaxKind::ConcurrentStatement => 78,
            SyntaxKind::SimpleInitializer => 79,
            SyntaxKind::AnonymousClass => 80,
            SyntaxKind::AnonymousFunction => 81,
            SyntaxKind::AnonymousFunctionUseClause => 82,
            SyntaxKind::LambdaExpression => 83,
            SyntaxKind::LambdaSignature => 84,
            SyntaxKind::CastExpression => 85,
            SyntaxKind::ScopeResolutionExpression => 86,
            SyntaxKind::MemberSelectionExpression => 87,
            SyntaxKind::SafeMemberSelectionExpression => 88,
            SyntaxKind::EmbeddedMemberSelectionExpression => 89,
            SyntaxKind::YieldExpression => 90,
            SyntaxKind::PrefixUnaryExpression => 91,
            SyntaxKind::PostfixUnaryExpression => 92,
            SyntaxKind::BinaryExpression => 93,
            SyntaxKind::IsExpression => 94,
            SyntaxKind::AsExpression => 95,
            SyntaxKind::NullableAsExpression => 96,
            SyntaxKind::UpcastExpression => 97,
            SyntaxKind::ConditionalExpression => 98,
            SyntaxKind::EvalExpression => 99,
            SyntaxKind::IssetExpression => 100,
            SyntaxKind::FunctionCallExpression => 101,
            SyntaxKind::FunctionPointerExpression => 102,
            SyntaxKind::ParenthesizedExpression => 103,
            SyntaxKind::BracedExpression => 104,
            SyntaxKind::ETSpliceExpression => 105,
            SyntaxKind::EmbeddedBracedExpression => 106,
            SyntaxKind::ListExpression => 107,
            SyntaxKind::CollectionLiteralExpression => 108,
            SyntaxKind::ObjectCreationExpression => 109,
            SyntaxKind::ConstructorCall => 110,
            SyntaxKind::DarrayIntrinsicExpression => 111,
            SyntaxKind::DictionaryIntrinsicExpression => 112,
            SyntaxKind::KeysetIntrinsicExpression => 113,
            SyntaxKind::VarrayIntrinsicExpression => 114,
            SyntaxKind::VectorIntrinsicExpression => 115,
            SyntaxKind::ElementInitializer => 116,
            SyntaxKind::SubscriptExpression => 117,
            SyntaxKind::EmbeddedSubscriptExpression => 118,
            SyntaxKind::AwaitableCreationExpression => 119,
            SyntaxKind::XHPChildrenDeclaration => 120,
            SyntaxKind::XHPChildrenParenthesizedList => 121,
            SyntaxKind::XHPCategoryDeclaration => 122,
            SyntaxKind::XHPEnumType => 123,
            SyntaxKind::XHPLateinit => 124,
            SyntaxKind::XHPRequired => 125,
            SyntaxKind::XHPClassAttributeDeclaration => 126,
            SyntaxKind::XHPClassAttribute => 127,
            SyntaxKind::XHPSimpleClassAttribute => 128,
            SyntaxKind::XHPSimpleAttribute => 129,
            SyntaxKind::XHPSpreadAttribute => 130,
            SyntaxKind::XHPOpen => 131,
            SyntaxKind::XHPExpression => 132,
            SyntaxKind::XHPClose => 133,
            SyntaxKind::TypeConstant => 134,
            SyntaxKind::VectorTypeSpecifier => 135,
            SyntaxKind::KeysetTypeSpecifier => 136,
            SyntaxKind::TupleTypeExplicitSpecifier => 137,
            SyntaxKind::VarrayTypeSpecifier => 138,
            SyntaxKind::FunctionCtxTypeSpecifier => 139,
            SyntaxKind::TypeParameter => 140,
            SyntaxKind::TypeConstraint => 141,
            SyntaxKind::ContextConstraint => 142,
            SyntaxKind::DarrayTypeSpecifier => 143,
            SyntaxKind::DictionaryTypeSpecifier => 144,
            SyntaxKind::ClosureTypeSpecifier => 145,
            SyntaxKind::ClosureParameterTypeSpecifier => 146,
            SyntaxKind::ClassnameTypeSpecifier => 147,
            SyntaxKind::FieldSpecifier => 148,
            SyntaxKind::FieldInitializer => 149,
            SyntaxKind::ShapeTypeSpecifier => 150,
            SyntaxKind::ShapeExpression => 151,
            SyntaxKind::TupleExpression => 152,
            SyntaxKind::GenericTypeSpecifier => 153,
            SyntaxKind::NullableTypeSpecifier => 154,
            SyntaxKind::LikeTypeSpecifier => 155,
            SyntaxKind::SoftTypeSpecifier => 156,
            SyntaxKind::AttributizedSpecifier => 157,
            SyntaxKind::ReifiedTypeArgument => 158,
            SyntaxKind::TypeArguments => 159,
            SyntaxKind::TypeParameters => 160,
            SyntaxKind::TupleTypeSpecifier => 161,
            SyntaxKind::UnionTypeSpecifier => 162,
            SyntaxKind::IntersectionTypeSpecifier => 163,
            SyntaxKind::ErrorSyntax => 164,
            SyntaxKind::ListItem => 165,
            SyntaxKind::EnumClassLabelExpression => 166,
            SyntaxKind::ModuleDeclaration => 167,
            SyntaxKind::ModuleMembershipDeclaration => 168,
        }
    }
}
