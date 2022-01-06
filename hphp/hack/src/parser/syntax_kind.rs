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
    TraitUsePrecedenceItem,
    TraitUseAliasItem,
    TraitUseConflictResolution,
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
    ElseifClause,
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
            SyntaxKind::TraitUsePrecedenceItem            => "trait_use_precedence_item",
            SyntaxKind::TraitUseAliasItem                 => "trait_use_alias_item",
            SyntaxKind::TraitUseConflictResolution        => "trait_use_conflict_resolution",
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
            SyntaxKind::ElseifClause                      => "elseif_clause",
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
            SyntaxKind::TraitUsePrecedenceItem => 37,
            SyntaxKind::TraitUseAliasItem => 38,
            SyntaxKind::TraitUseConflictResolution => 39,
            SyntaxKind::TraitUse => 40,
            SyntaxKind::RequireClause => 41,
            SyntaxKind::ConstDeclaration => 42,
            SyntaxKind::ConstantDeclarator => 43,
            SyntaxKind::TypeConstDeclaration => 44,
            SyntaxKind::ContextConstDeclaration => 45,
            SyntaxKind::DecoratedExpression => 46,
            SyntaxKind::ParameterDeclaration => 47,
            SyntaxKind::VariadicParameter => 48,
            SyntaxKind::OldAttributeSpecification => 49,
            SyntaxKind::AttributeSpecification => 50,
            SyntaxKind::Attribute => 51,
            SyntaxKind::InclusionExpression => 52,
            SyntaxKind::InclusionDirective => 53,
            SyntaxKind::CompoundStatement => 54,
            SyntaxKind::ExpressionStatement => 55,
            SyntaxKind::MarkupSection => 56,
            SyntaxKind::MarkupSuffix => 57,
            SyntaxKind::UnsetStatement => 58,
            SyntaxKind::UsingStatementBlockScoped => 59,
            SyntaxKind::UsingStatementFunctionScoped => 60,
            SyntaxKind::WhileStatement => 61,
            SyntaxKind::IfStatement => 62,
            SyntaxKind::ElseifClause => 63,
            SyntaxKind::ElseClause => 64,
            SyntaxKind::TryStatement => 65,
            SyntaxKind::CatchClause => 66,
            SyntaxKind::FinallyClause => 67,
            SyntaxKind::DoStatement => 68,
            SyntaxKind::ForStatement => 69,
            SyntaxKind::ForeachStatement => 70,
            SyntaxKind::SwitchStatement => 71,
            SyntaxKind::SwitchSection => 72,
            SyntaxKind::SwitchFallthrough => 73,
            SyntaxKind::CaseLabel => 74,
            SyntaxKind::DefaultLabel => 75,
            SyntaxKind::ReturnStatement => 76,
            SyntaxKind::YieldBreakStatement => 77,
            SyntaxKind::ThrowStatement => 78,
            SyntaxKind::BreakStatement => 79,
            SyntaxKind::ContinueStatement => 80,
            SyntaxKind::EchoStatement => 81,
            SyntaxKind::ConcurrentStatement => 82,
            SyntaxKind::SimpleInitializer => 83,
            SyntaxKind::AnonymousClass => 84,
            SyntaxKind::AnonymousFunction => 85,
            SyntaxKind::AnonymousFunctionUseClause => 86,
            SyntaxKind::LambdaExpression => 87,
            SyntaxKind::LambdaSignature => 88,
            SyntaxKind::CastExpression => 89,
            SyntaxKind::ScopeResolutionExpression => 90,
            SyntaxKind::MemberSelectionExpression => 91,
            SyntaxKind::SafeMemberSelectionExpression => 92,
            SyntaxKind::EmbeddedMemberSelectionExpression => 93,
            SyntaxKind::YieldExpression => 94,
            SyntaxKind::PrefixUnaryExpression => 95,
            SyntaxKind::PostfixUnaryExpression => 96,
            SyntaxKind::BinaryExpression => 97,
            SyntaxKind::IsExpression => 98,
            SyntaxKind::AsExpression => 99,
            SyntaxKind::NullableAsExpression => 100,
            SyntaxKind::UpcastExpression => 101,
            SyntaxKind::ConditionalExpression => 102,
            SyntaxKind::EvalExpression => 103,
            SyntaxKind::IssetExpression => 104,
            SyntaxKind::FunctionCallExpression => 105,
            SyntaxKind::FunctionPointerExpression => 106,
            SyntaxKind::ParenthesizedExpression => 107,
            SyntaxKind::BracedExpression => 108,
            SyntaxKind::ETSpliceExpression => 109,
            SyntaxKind::EmbeddedBracedExpression => 110,
            SyntaxKind::ListExpression => 111,
            SyntaxKind::CollectionLiteralExpression => 112,
            SyntaxKind::ObjectCreationExpression => 113,
            SyntaxKind::ConstructorCall => 114,
            SyntaxKind::DarrayIntrinsicExpression => 115,
            SyntaxKind::DictionaryIntrinsicExpression => 116,
            SyntaxKind::KeysetIntrinsicExpression => 117,
            SyntaxKind::VarrayIntrinsicExpression => 118,
            SyntaxKind::VectorIntrinsicExpression => 119,
            SyntaxKind::ElementInitializer => 120,
            SyntaxKind::SubscriptExpression => 121,
            SyntaxKind::EmbeddedSubscriptExpression => 122,
            SyntaxKind::AwaitableCreationExpression => 123,
            SyntaxKind::XHPChildrenDeclaration => 124,
            SyntaxKind::XHPChildrenParenthesizedList => 125,
            SyntaxKind::XHPCategoryDeclaration => 126,
            SyntaxKind::XHPEnumType => 127,
            SyntaxKind::XHPLateinit => 128,
            SyntaxKind::XHPRequired => 129,
            SyntaxKind::XHPClassAttributeDeclaration => 130,
            SyntaxKind::XHPClassAttribute => 131,
            SyntaxKind::XHPSimpleClassAttribute => 132,
            SyntaxKind::XHPSimpleAttribute => 133,
            SyntaxKind::XHPSpreadAttribute => 134,
            SyntaxKind::XHPOpen => 135,
            SyntaxKind::XHPExpression => 136,
            SyntaxKind::XHPClose => 137,
            SyntaxKind::TypeConstant => 138,
            SyntaxKind::VectorTypeSpecifier => 139,
            SyntaxKind::KeysetTypeSpecifier => 140,
            SyntaxKind::TupleTypeExplicitSpecifier => 141,
            SyntaxKind::VarrayTypeSpecifier => 142,
            SyntaxKind::FunctionCtxTypeSpecifier => 143,
            SyntaxKind::TypeParameter => 144,
            SyntaxKind::TypeConstraint => 145,
            SyntaxKind::ContextConstraint => 146,
            SyntaxKind::DarrayTypeSpecifier => 147,
            SyntaxKind::DictionaryTypeSpecifier => 148,
            SyntaxKind::ClosureTypeSpecifier => 149,
            SyntaxKind::ClosureParameterTypeSpecifier => 150,
            SyntaxKind::ClassnameTypeSpecifier => 151,
            SyntaxKind::FieldSpecifier => 152,
            SyntaxKind::FieldInitializer => 153,
            SyntaxKind::ShapeTypeSpecifier => 154,
            SyntaxKind::ShapeExpression => 155,
            SyntaxKind::TupleExpression => 156,
            SyntaxKind::GenericTypeSpecifier => 157,
            SyntaxKind::NullableTypeSpecifier => 158,
            SyntaxKind::LikeTypeSpecifier => 159,
            SyntaxKind::SoftTypeSpecifier => 160,
            SyntaxKind::AttributizedSpecifier => 161,
            SyntaxKind::ReifiedTypeArgument => 162,
            SyntaxKind::TypeArguments => 163,
            SyntaxKind::TypeParameters => 164,
            SyntaxKind::TupleTypeSpecifier => 165,
            SyntaxKind::UnionTypeSpecifier => 166,
            SyntaxKind::IntersectionTypeSpecifier => 167,
            SyntaxKind::ErrorSyntax => 168,
            SyntaxKind::ListItem => 169,
            SyntaxKind::EnumClassLabelExpression => 170,
        }
    }
}
