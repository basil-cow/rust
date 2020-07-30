//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{
    ast::{self, support, AstChildren, AstNode},
    SyntaxKind::{self, *},
    SyntaxNode, SyntaxToken, T,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceFile {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for SourceFile {}
impl ast::ModuleItemOwner for SourceFile {}
impl SourceFile {
    pub fn shebang_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![shebang]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attr {
    pub(crate) syntax: SyntaxNode,
}
impl Attr {
    pub fn pound_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![#]) }
    pub fn excl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![!]) }
    pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn input(&self) -> Option<AttrInput> { support::child(&self.syntax) }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Const {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Const {}
impl ast::NameOwner for Const {}
impl ast::VisibilityOwner for Const {}
impl ast::TypeAscriptionOwner for Const {}
impl Const {
    pub fn default_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![default]) }
    pub fn const_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![const]) }
    pub fn underscore_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![_]) }
    pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![:]) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn body(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Enum {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Enum {}
impl ast::NameOwner for Enum {}
impl ast::VisibilityOwner for Enum {}
impl ast::GenericParamsOwner for Enum {}
impl Enum {
    pub fn enum_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![enum]) }
    pub fn variant_list(&self) -> Option<VariantList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternBlock {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ExternBlock {}
impl ExternBlock {
    pub fn abi(&self) -> Option<Abi> { support::child(&self.syntax) }
    pub fn extern_item_list(&self) -> Option<ExternItemList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternCrate {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ExternCrate {}
impl ast::VisibilityOwner for ExternCrate {}
impl ExternCrate {
    pub fn extern_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![extern]) }
    pub fn crate_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![crate]) }
    pub fn name_ref(&self) -> Option<NameRef> { support::child(&self.syntax) }
    pub fn self_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![self]) }
    pub fn rename(&self) -> Option<Rename> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Fn {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Fn {}
impl ast::NameOwner for Fn {}
impl ast::VisibilityOwner for Fn {}
impl ast::GenericParamsOwner for Fn {}
impl Fn {
    pub fn default_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![default]) }
    pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
    pub fn const_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![const]) }
    pub fn unsafe_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![unsafe]) }
    pub fn abi(&self) -> Option<Abi> { support::child(&self.syntax) }
    pub fn fn_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![fn]) }
    pub fn param_list(&self) -> Option<ParamList> { support::child(&self.syntax) }
    pub fn ret_type(&self) -> Option<RetType> { support::child(&self.syntax) }
    pub fn body(&self) -> Option<BlockExpr> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Impl {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Impl {}
impl ast::VisibilityOwner for Impl {}
impl ast::GenericParamsOwner for Impl {}
impl Impl {
    pub fn default_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![default]) }
    pub fn unsafe_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![unsafe]) }
    pub fn impl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![impl]) }
    pub fn const_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![const]) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
    pub fn excl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![!]) }
    pub fn for_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![for]) }
    pub fn assoc_item_list(&self) -> Option<AssocItemList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroCall {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for MacroCall {}
impl ast::NameOwner for MacroCall {}
impl MacroCall {
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
    pub fn excl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![!]) }
    pub fn token_tree(&self) -> Option<TokenTree> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Module {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Module {}
impl ast::NameOwner for Module {}
impl ast::VisibilityOwner for Module {}
impl Module {
    pub fn mod_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![mod]) }
    pub fn item_list(&self) -> Option<ItemList> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Static {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Static {}
impl ast::NameOwner for Static {}
impl ast::VisibilityOwner for Static {}
impl ast::TypeAscriptionOwner for Static {}
impl Static {
    pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
    pub fn mut_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![mut]) }
    pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![:]) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn body(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Struct {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Struct {}
impl ast::NameOwner for Struct {}
impl ast::VisibilityOwner for Struct {}
impl ast::GenericParamsOwner for Struct {}
impl Struct {
    pub fn struct_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![struct]) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
    pub fn field_list(&self) -> Option<FieldList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Trait {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Trait {}
impl ast::NameOwner for Trait {}
impl ast::VisibilityOwner for Trait {}
impl ast::GenericParamsOwner for Trait {}
impl ast::TypeBoundsOwner for Trait {}
impl Trait {
    pub fn unsafe_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![unsafe]) }
    pub fn auto_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![auto]) }
    pub fn trait_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![trait]) }
    pub fn assoc_item_list(&self) -> Option<AssocItemList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeAlias {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for TypeAlias {}
impl ast::NameOwner for TypeAlias {}
impl ast::VisibilityOwner for TypeAlias {}
impl ast::GenericParamsOwner for TypeAlias {}
impl ast::TypeBoundsOwner for TypeAlias {}
impl TypeAlias {
    pub fn default_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![default]) }
    pub fn type_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![type]) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Union {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Union {}
impl ast::NameOwner for Union {}
impl ast::VisibilityOwner for Union {}
impl ast::GenericParamsOwner for Union {}
impl Union {
    pub fn union_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![union]) }
    pub fn record_field_list(&self) -> Option<RecordFieldList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Use {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Use {}
impl ast::VisibilityOwner for Use {}
impl Use {
    pub fn use_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![use]) }
    pub fn use_tree(&self) -> Option<UseTree> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Visibility {
    pub(crate) syntax: SyntaxNode,
}
impl Visibility {
    pub fn pub_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![pub]) }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn super_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![super]) }
    pub fn self_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![self]) }
    pub fn crate_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![crate]) }
    pub fn in_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![in]) }
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Name {
    pub(crate) syntax: SyntaxNode,
}
impl Name {
    pub fn ident_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![ident]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ItemList {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ItemList {}
impl ast::ModuleItemOwner for ItemList {}
impl ItemList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NameRef {
    pub(crate) syntax: SyntaxNode,
}
impl NameRef {
    pub fn ident_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![ident]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rename {
    pub(crate) syntax: SyntaxNode,
}
impl ast::NameOwner for Rename {}
impl Rename {
    pub fn as_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![as]) }
    pub fn underscore_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![_]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseTree {
    pub(crate) syntax: SyntaxNode,
}
impl UseTree {
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
    pub fn coloncolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![::]) }
    pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![*]) }
    pub fn use_tree_list(&self) -> Option<UseTreeList> { support::child(&self.syntax) }
    pub fn rename(&self) -> Option<Rename> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    pub(crate) syntax: SyntaxNode,
}
impl Path {
    pub fn qualifier(&self) -> Option<Path> { support::child(&self.syntax) }
    pub fn coloncolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![::]) }
    pub fn segment(&self) -> Option<PathSegment> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UseTreeList {
    pub(crate) syntax: SyntaxNode,
}
impl UseTreeList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn use_trees(&self) -> AstChildren<UseTree> { support::children(&self.syntax) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Abi {
    pub(crate) syntax: SyntaxNode,
}
impl Abi {
    pub fn extern_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![extern]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericParamList {
    pub(crate) syntax: SyntaxNode,
}
impl GenericParamList {
    pub fn l_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![<]) }
    pub fn generic_params(&self) -> AstChildren<GenericParam> { support::children(&self.syntax) }
    pub fn r_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![>]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParamList {
    pub(crate) syntax: SyntaxNode,
}
impl ParamList {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn params(&self) -> AstChildren<Param> { support::children(&self.syntax) }
    pub fn self_param(&self) -> Option<SelfParam> { support::child(&self.syntax) }
    pub fn comma_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![,]) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RetType {
    pub(crate) syntax: SyntaxNode,
}
impl RetType {
    pub fn thin_arrow_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![->]) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhereClause {
    pub(crate) syntax: SyntaxNode,
}
impl WhereClause {
    pub fn where_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![where]) }
    pub fn predicates(&self) -> AstChildren<WherePred> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BlockExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for BlockExpr {}
impl ast::ModuleItemOwner for BlockExpr {}
impl BlockExpr {
    pub fn label(&self) -> Option<Label> { support::child(&self.syntax) }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn statements(&self) -> AstChildren<Stmt> { support::children(&self.syntax) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Param {}
impl ast::TypeAscriptionOwner for Param {}
impl Param {
    pub fn pat(&self) -> Option<Pat> { support::child(&self.syntax) }
    pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![:]) }
    pub fn dotdotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![...]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SelfParam {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for SelfParam {}
impl ast::TypeAscriptionOwner for SelfParam {}
impl SelfParam {
    pub fn amp_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![&]) }
    pub fn lifetime_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![lifetime])
    }
    pub fn mut_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![mut]) }
    pub fn self_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![self]) }
    pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![:]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeBoundList {
    pub(crate) syntax: SyntaxNode,
}
impl TypeBoundList {
    pub fn bounds(&self) -> AstChildren<TypeBound> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldList {
    pub(crate) syntax: SyntaxNode,
}
impl RecordFieldList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn fields(&self) -> AstChildren<RecordField> { support::children(&self.syntax) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleFieldList {
    pub(crate) syntax: SyntaxNode,
}
impl TupleFieldList {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn fields(&self) -> AstChildren<TupleField> { support::children(&self.syntax) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordField {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for RecordField {}
impl ast::NameOwner for RecordField {}
impl ast::VisibilityOwner for RecordField {}
impl ast::TypeAscriptionOwner for RecordField {}
impl RecordField {
    pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![:]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleField {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for TupleField {}
impl ast::VisibilityOwner for TupleField {}
impl TupleField {
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariantList {
    pub(crate) syntax: SyntaxNode,
}
impl VariantList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn variants(&self) -> AstChildren<Variant> { support::children(&self.syntax) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for Variant {}
impl ast::NameOwner for Variant {}
impl ast::VisibilityOwner for Variant {}
impl Variant {
    pub fn field_list(&self) -> Option<FieldList> { support::child(&self.syntax) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssocItemList {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for AssocItemList {}
impl AssocItemList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn assoc_items(&self) -> AstChildren<AssocItem> { support::children(&self.syntax) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExternItemList {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ExternItemList {}
impl ExternItemList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn extern_items(&self) -> AstChildren<ExternItem> { support::children(&self.syntax) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LifetimeParam {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for LifetimeParam {}
impl LifetimeParam {
    pub fn lifetime_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![lifetime])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeParam {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for TypeParam {}
impl ast::NameOwner for TypeParam {}
impl ast::TypeBoundsOwner for TypeParam {}
impl TypeParam {
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn default_type(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstParam {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ConstParam {}
impl ast::NameOwner for ConstParam {}
impl ast::TypeAscriptionOwner for ConstParam {}
impl ConstParam {
    pub fn const_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![const]) }
    pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![:]) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn default_val(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenType {
    pub(crate) syntax: SyntaxNode,
}
impl ParenType {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleType {
    pub(crate) syntax: SyntaxNode,
}
impl TupleType {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn fields(&self) -> AstChildren<TypeRef> { support::children(&self.syntax) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NeverType {
    pub(crate) syntax: SyntaxNode,
}
impl NeverType {
    pub fn excl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![!]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathType {
    pub(crate) syntax: SyntaxNode,
}
impl PathType {
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PointerType {
    pub(crate) syntax: SyntaxNode,
}
impl PointerType {
    pub fn star_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![*]) }
    pub fn const_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![const]) }
    pub fn mut_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![mut]) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayType {
    pub(crate) syntax: SyntaxNode,
}
impl ArrayType {
    pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SliceType {
    pub(crate) syntax: SyntaxNode,
}
impl SliceType {
    pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReferenceType {
    pub(crate) syntax: SyntaxNode,
}
impl ReferenceType {
    pub fn amp_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![&]) }
    pub fn lifetime_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![lifetime])
    }
    pub fn mut_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![mut]) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaceholderType {
    pub(crate) syntax: SyntaxNode,
}
impl PlaceholderType {
    pub fn underscore_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![_]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FnPointerType {
    pub(crate) syntax: SyntaxNode,
}
impl FnPointerType {
    pub fn abi(&self) -> Option<Abi> { support::child(&self.syntax) }
    pub fn unsafe_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![unsafe]) }
    pub fn fn_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![fn]) }
    pub fn param_list(&self) -> Option<ParamList> { support::child(&self.syntax) }
    pub fn ret_type(&self) -> Option<RetType> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForType {
    pub(crate) syntax: SyntaxNode,
}
impl ForType {
    pub fn for_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![for]) }
    pub fn generic_param_list(&self) -> Option<GenericParamList> { support::child(&self.syntax) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImplTraitType {
    pub(crate) syntax: SyntaxNode,
}
impl ImplTraitType {
    pub fn impl_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![impl]) }
    pub fn type_bound_list(&self) -> Option<TypeBoundList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DynTraitType {
    pub(crate) syntax: SyntaxNode,
}
impl DynTraitType {
    pub fn dyn_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![dyn]) }
    pub fn type_bound_list(&self) -> Option<TypeBoundList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for TupleExpr {}
impl TupleExpr {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn exprs(&self) -> AstChildren<Expr> { support::children(&self.syntax) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArrayExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ArrayExpr {}
impl ArrayExpr {
    pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
    pub fn exprs(&self) -> AstChildren<Expr> { support::children(&self.syntax) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ParenExpr {}
impl ParenExpr {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathExpr {
    pub(crate) syntax: SyntaxNode,
}
impl PathExpr {
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LambdaExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for LambdaExpr {}
impl LambdaExpr {
    pub fn static_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![static]) }
    pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
    pub fn move_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![move]) }
    pub fn param_list(&self) -> Option<ParamList> { support::child(&self.syntax) }
    pub fn ret_type(&self) -> Option<RetType> { support::child(&self.syntax) }
    pub fn body(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IfExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for IfExpr {}
impl IfExpr {
    pub fn if_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![if]) }
    pub fn condition(&self) -> Option<Condition> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Condition {
    pub(crate) syntax: SyntaxNode,
}
impl Condition {
    pub fn let_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![let]) }
    pub fn pat(&self) -> Option<Pat> { support::child(&self.syntax) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EffectExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for EffectExpr {}
impl EffectExpr {
    pub fn label(&self) -> Option<Label> { support::child(&self.syntax) }
    pub fn try_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![try]) }
    pub fn unsafe_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![unsafe]) }
    pub fn async_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![async]) }
    pub fn block_expr(&self) -> Option<BlockExpr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Label {
    pub(crate) syntax: SyntaxNode,
}
impl Label {
    pub fn lifetime_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![lifetime])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoopExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for LoopExpr {}
impl ast::LoopBodyOwner for LoopExpr {}
impl LoopExpr {
    pub fn loop_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![loop]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ForExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ForExpr {}
impl ast::LoopBodyOwner for ForExpr {}
impl ForExpr {
    pub fn for_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![for]) }
    pub fn pat(&self) -> Option<Pat> { support::child(&self.syntax) }
    pub fn in_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![in]) }
    pub fn iterable(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WhileExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for WhileExpr {}
impl ast::LoopBodyOwner for WhileExpr {}
impl WhileExpr {
    pub fn while_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![while]) }
    pub fn condition(&self) -> Option<Condition> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ContinueExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ContinueExpr {}
impl ContinueExpr {
    pub fn continue_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![continue])
    }
    pub fn lifetime_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![lifetime])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for BreakExpr {}
impl BreakExpr {
    pub fn break_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![break]) }
    pub fn lifetime_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![lifetime])
    }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReturnExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ReturnExpr {}
impl ReturnExpr {
    pub fn return_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![return]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for CallExpr {}
impl ast::ArgListOwner for CallExpr {}
impl CallExpr {
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ArgList {
    pub(crate) syntax: SyntaxNode,
}
impl ArgList {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn args(&self) -> AstChildren<Expr> { support::children(&self.syntax) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodCallExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for MethodCallExpr {}
impl ast::ArgListOwner for MethodCallExpr {}
impl MethodCallExpr {
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![.]) }
    pub fn name_ref(&self) -> Option<NameRef> { support::child(&self.syntax) }
    pub fn type_arg_list(&self) -> Option<TypeArgList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeArgList {
    pub(crate) syntax: SyntaxNode,
}
impl TypeArgList {
    pub fn coloncolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![::]) }
    pub fn l_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![<]) }
    pub fn type_args(&self) -> AstChildren<TypeArg> { support::children(&self.syntax) }
    pub fn lifetime_args(&self) -> AstChildren<LifetimeArg> { support::children(&self.syntax) }
    pub fn assoc_type_args(&self) -> AstChildren<AssocTypeArg> { support::children(&self.syntax) }
    pub fn const_args(&self) -> AstChildren<ConstArg> { support::children(&self.syntax) }
    pub fn r_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![>]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for FieldExpr {}
impl FieldExpr {
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![.]) }
    pub fn name_ref(&self) -> Option<NameRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IndexExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for IndexExpr {}
impl IndexExpr {
    pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AwaitExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for AwaitExpr {}
impl AwaitExpr {
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn dot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![.]) }
    pub fn await_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![await]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TryExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for TryExpr {}
impl TryExpr {
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn question_mark_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![?]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CastExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for CastExpr {}
impl CastExpr {
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn as_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![as]) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for RefExpr {}
impl RefExpr {
    pub fn amp_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![&]) }
    pub fn raw_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![raw]) }
    pub fn mut_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![mut]) }
    pub fn const_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![const]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PrefixExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for PrefixExpr {}
impl PrefixExpr {
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoxExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for BoxExpr {}
impl BoxExpr {
    pub fn box_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![box]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RangeExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for RangeExpr {}
impl RangeExpr {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BinExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for BinExpr {}
impl BinExpr {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub(crate) syntax: SyntaxNode,
}
impl Literal {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchExpr {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for MatchExpr {}
impl MatchExpr {
    pub fn match_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![match]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn match_arm_list(&self) -> Option<MatchArmList> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArmList {
    pub(crate) syntax: SyntaxNode,
}
impl MatchArmList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn arms(&self) -> AstChildren<MatchArm> { support::children(&self.syntax) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArm {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for MatchArm {}
impl MatchArm {
    pub fn pat(&self) -> Option<Pat> { support::child(&self.syntax) }
    pub fn guard(&self) -> Option<MatchGuard> { support::child(&self.syntax) }
    pub fn fat_arrow_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=>]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchGuard {
    pub(crate) syntax: SyntaxNode,
}
impl MatchGuard {
    pub fn if_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![if]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordExpr {
    pub(crate) syntax: SyntaxNode,
}
impl RecordExpr {
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
    pub fn record_expr_field_list(&self) -> Option<RecordExprFieldList> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordExprFieldList {
    pub(crate) syntax: SyntaxNode,
}
impl RecordExprFieldList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn fields(&self) -> AstChildren<RecordExprField> { support::children(&self.syntax) }
    pub fn dotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![..]) }
    pub fn spread(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordExprField {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for RecordExprField {}
impl RecordExprField {
    pub fn name_ref(&self) -> Option<NameRef> { support::child(&self.syntax) }
    pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![:]) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrPat {
    pub(crate) syntax: SyntaxNode,
}
impl OrPat {
    pub fn pats(&self) -> AstChildren<Pat> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParenPat {
    pub(crate) syntax: SyntaxNode,
}
impl ParenPat {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn pat(&self) -> Option<Pat> { support::child(&self.syntax) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RefPat {
    pub(crate) syntax: SyntaxNode,
}
impl RefPat {
    pub fn amp_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![&]) }
    pub fn mut_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![mut]) }
    pub fn pat(&self) -> Option<Pat> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoxPat {
    pub(crate) syntax: SyntaxNode,
}
impl BoxPat {
    pub fn box_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![box]) }
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindPat {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for BindPat {}
impl ast::NameOwner for BindPat {}
impl BindPat {
    pub fn ref_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![ref]) }
    pub fn mut_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![mut]) }
    pub fn at_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![@]) }
    pub fn pat(&self) -> Option<Pat> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlaceholderPat {
    pub(crate) syntax: SyntaxNode,
}
impl PlaceholderPat {
    pub fn underscore_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![_]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DotDotPat {
    pub(crate) syntax: SyntaxNode,
}
impl DotDotPat {
    pub fn dotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![..]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathPat {
    pub(crate) syntax: SyntaxNode,
}
impl PathPat {
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SlicePat {
    pub(crate) syntax: SyntaxNode,
}
impl SlicePat {
    pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
    pub fn args(&self) -> AstChildren<Pat> { support::children(&self.syntax) }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RangePat {
    pub(crate) syntax: SyntaxNode,
}
impl RangePat {
    pub fn dotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![..]) }
    pub fn dotdoteq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![..=]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LiteralPat {
    pub(crate) syntax: SyntaxNode,
}
impl LiteralPat {
    pub fn literal(&self) -> Option<Literal> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroPat {
    pub(crate) syntax: SyntaxNode,
}
impl MacroPat {
    pub fn macro_call(&self) -> Option<MacroCall> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordPat {
    pub(crate) syntax: SyntaxNode,
}
impl RecordPat {
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
    pub fn record_field_pat_list(&self) -> Option<RecordFieldPatList> {
        support::child(&self.syntax)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldPatList {
    pub(crate) syntax: SyntaxNode,
}
impl RecordFieldPatList {
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn record_field_pats(&self) -> AstChildren<RecordFieldPat> {
        support::children(&self.syntax)
    }
    pub fn bind_pats(&self) -> AstChildren<BindPat> { support::children(&self.syntax) }
    pub fn dotdot_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![..]) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordFieldPat {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for RecordFieldPat {}
impl RecordFieldPat {
    pub fn name_ref(&self) -> Option<NameRef> { support::child(&self.syntax) }
    pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![:]) }
    pub fn pat(&self) -> Option<Pat> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TupleStructPat {
    pub(crate) syntax: SyntaxNode,
}
impl TupleStructPat {
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn args(&self) -> AstChildren<Pat> { support::children(&self.syntax) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TuplePat {
    pub(crate) syntax: SyntaxNode,
}
impl TuplePat {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn args(&self) -> AstChildren<Pat> { support::children(&self.syntax) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenTree {
    pub(crate) syntax: SyntaxNode,
}
impl TokenTree {
    pub fn l_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['(']) }
    pub fn r_paren_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![')']) }
    pub fn l_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['{']) }
    pub fn r_curly_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['}']) }
    pub fn l_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T!['[']) }
    pub fn r_brack_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![']']) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroDef {
    pub(crate) syntax: SyntaxNode,
}
impl ast::NameOwner for MacroDef {}
impl MacroDef {
    pub fn token_tree(&self) -> Option<TokenTree> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroItems {
    pub(crate) syntax: SyntaxNode,
}
impl ast::ModuleItemOwner for MacroItems {}
impl MacroItems {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MacroStmts {
    pub(crate) syntax: SyntaxNode,
}
impl MacroStmts {
    pub fn statements(&self) -> AstChildren<Stmt> { support::children(&self.syntax) }
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeBound {
    pub(crate) syntax: SyntaxNode,
}
impl TypeBound {
    pub fn lifetime_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![lifetime])
    }
    pub fn const_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![const]) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WherePred {
    pub(crate) syntax: SyntaxNode,
}
impl ast::TypeBoundsOwner for WherePred {}
impl WherePred {
    pub fn for_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![for]) }
    pub fn generic_param_list(&self) -> Option<GenericParamList> { support::child(&self.syntax) }
    pub fn lifetime_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![lifetime])
    }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExprStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for ExprStmt {}
impl ExprStmt {
    pub fn expr(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LetStmt {
    pub(crate) syntax: SyntaxNode,
}
impl ast::AttrsOwner for LetStmt {}
impl ast::TypeAscriptionOwner for LetStmt {}
impl LetStmt {
    pub fn let_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![let]) }
    pub fn pat(&self) -> Option<Pat> { support::child(&self.syntax) }
    pub fn colon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![:]) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn initializer(&self) -> Option<Expr> { support::child(&self.syntax) }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![;]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PathSegment {
    pub(crate) syntax: SyntaxNode,
}
impl PathSegment {
    pub fn coloncolon_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![::]) }
    pub fn crate_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![crate]) }
    pub fn self_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![self]) }
    pub fn super_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![super]) }
    pub fn l_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![<]) }
    pub fn name_ref(&self) -> Option<NameRef> { support::child(&self.syntax) }
    pub fn type_arg_list(&self) -> Option<TypeArgList> { support::child(&self.syntax) }
    pub fn param_list(&self) -> Option<ParamList> { support::child(&self.syntax) }
    pub fn ret_type(&self) -> Option<RetType> { support::child(&self.syntax) }
    pub fn path_type(&self) -> Option<PathType> { support::child(&self.syntax) }
    pub fn r_angle_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![>]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeArg {
    pub(crate) syntax: SyntaxNode,
}
impl TypeArg {
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LifetimeArg {
    pub(crate) syntax: SyntaxNode,
}
impl LifetimeArg {
    pub fn lifetime_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, T![lifetime])
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AssocTypeArg {
    pub(crate) syntax: SyntaxNode,
}
impl ast::TypeBoundsOwner for AssocTypeArg {}
impl AssocTypeArg {
    pub fn name_ref(&self) -> Option<NameRef> { support::child(&self.syntax) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn type_ref(&self) -> Option<TypeRef> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstArg {
    pub(crate) syntax: SyntaxNode,
}
impl ConstArg {
    pub fn literal(&self) -> Option<Literal> { support::child(&self.syntax) }
    pub fn block_expr(&self) -> Option<BlockExpr> { support::child(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MetaItem {
    pub(crate) syntax: SyntaxNode,
}
impl MetaItem {
    pub fn path(&self) -> Option<Path> { support::child(&self.syntax) }
    pub fn eq_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![=]) }
    pub fn attr_input(&self) -> Option<AttrInput> { support::child(&self.syntax) }
    pub fn nested_meta_items(&self) -> AstChildren<MetaItem> { support::children(&self.syntax) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Const(Const),
    Enum(Enum),
    ExternBlock(ExternBlock),
    ExternCrate(ExternCrate),
    Fn(Fn),
    Impl(Impl),
    MacroCall(MacroCall),
    Module(Module),
    Static(Static),
    Struct(Struct),
    Trait(Trait),
    TypeAlias(TypeAlias),
    Union(Union),
    Use(Use),
}
impl ast::AttrsOwner for Item {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeRef {
    ParenType(ParenType),
    TupleType(TupleType),
    NeverType(NeverType),
    PathType(PathType),
    PointerType(PointerType),
    ArrayType(ArrayType),
    SliceType(SliceType),
    ReferenceType(ReferenceType),
    PlaceholderType(PlaceholderType),
    FnPointerType(FnPointerType),
    ForType(ForType),
    ImplTraitType(ImplTraitType),
    DynTraitType(DynTraitType),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pat {
    OrPat(OrPat),
    ParenPat(ParenPat),
    RefPat(RefPat),
    BoxPat(BoxPat),
    BindPat(BindPat),
    PlaceholderPat(PlaceholderPat),
    DotDotPat(DotDotPat),
    PathPat(PathPat),
    RecordPat(RecordPat),
    TupleStructPat(TupleStructPat),
    TuplePat(TuplePat),
    SlicePat(SlicePat),
    RangePat(RangePat),
    LiteralPat(LiteralPat),
    MacroPat(MacroPat),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldList {
    RecordFieldList(RecordFieldList),
    TupleFieldList(TupleFieldList),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    TupleExpr(TupleExpr),
    ArrayExpr(ArrayExpr),
    ParenExpr(ParenExpr),
    PathExpr(PathExpr),
    LambdaExpr(LambdaExpr),
    IfExpr(IfExpr),
    LoopExpr(LoopExpr),
    ForExpr(ForExpr),
    WhileExpr(WhileExpr),
    ContinueExpr(ContinueExpr),
    BreakExpr(BreakExpr),
    Label(Label),
    BlockExpr(BlockExpr),
    ReturnExpr(ReturnExpr),
    MatchExpr(MatchExpr),
    RecordExpr(RecordExpr),
    CallExpr(CallExpr),
    IndexExpr(IndexExpr),
    MethodCallExpr(MethodCallExpr),
    FieldExpr(FieldExpr),
    AwaitExpr(AwaitExpr),
    TryExpr(TryExpr),
    EffectExpr(EffectExpr),
    CastExpr(CastExpr),
    RefExpr(RefExpr),
    PrefixExpr(PrefixExpr),
    RangeExpr(RangeExpr),
    BinExpr(BinExpr),
    Literal(Literal),
    MacroCall(MacroCall),
    BoxExpr(BoxExpr),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AssocItem {
    Fn(Fn),
    TypeAlias(TypeAlias),
    Const(Const),
    MacroCall(MacroCall),
}
impl ast::AttrsOwner for AssocItem {}
impl ast::NameOwner for AssocItem {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExternItem {
    Fn(Fn),
    Static(Static),
    MacroCall(MacroCall),
}
impl ast::AttrsOwner for ExternItem {}
impl ast::NameOwner for ExternItem {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GenericParam {
    LifetimeParam(LifetimeParam),
    TypeParam(TypeParam),
    ConstParam(ConstParam),
}
impl ast::AttrsOwner for GenericParam {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    LetStmt(LetStmt),
    ExprStmt(ExprStmt),
}
impl ast::AttrsOwner for Stmt {}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttrInput {
    Literal(Literal),
    TokenTree(TokenTree),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdtDef {
    Struct(Struct),
    Enum(Enum),
    Union(Union),
}
impl ast::AttrsOwner for AdtDef {}
impl ast::GenericParamsOwner for AdtDef {}
impl ast::NameOwner for AdtDef {}
impl ast::VisibilityOwner for AdtDef {}
impl AstNode for SourceFile {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SOURCE_FILE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Attr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ATTR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Const {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CONST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Enum {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ENUM }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExternBlock {
    fn can_cast(kind: SyntaxKind) -> bool { kind == EXTERN_BLOCK }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExternCrate {
    fn can_cast(kind: SyntaxKind) -> bool { kind == EXTERN_CRATE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Fn {
    fn can_cast(kind: SyntaxKind) -> bool { kind == FN }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Impl {
    fn can_cast(kind: SyntaxKind) -> bool { kind == IMPL }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MacroCall {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MACRO_CALL }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Module {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MODULE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Static {
    fn can_cast(kind: SyntaxKind) -> bool { kind == STATIC }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Struct {
    fn can_cast(kind: SyntaxKind) -> bool { kind == STRUCT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Trait {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TRAIT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TypeAlias {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TYPE_ALIAS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Union {
    fn can_cast(kind: SyntaxKind) -> bool { kind == UNION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Use {
    fn can_cast(kind: SyntaxKind) -> bool { kind == USE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Visibility {
    fn can_cast(kind: SyntaxKind) -> bool { kind == VISIBILITY }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Name {
    fn can_cast(kind: SyntaxKind) -> bool { kind == NAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ItemList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ITEM_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for NameRef {
    fn can_cast(kind: SyntaxKind) -> bool { kind == NAME_REF }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Rename {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RENAME }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for UseTree {
    fn can_cast(kind: SyntaxKind) -> bool { kind == USE_TREE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Path {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PATH }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for UseTreeList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == USE_TREE_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Abi {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ABI }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for GenericParamList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == GENERIC_PARAM_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ParamList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PARAM_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RetType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RET_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for WhereClause {
    fn can_cast(kind: SyntaxKind) -> bool { kind == WHERE_CLAUSE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BlockExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BLOCK_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Param {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PARAM }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SelfParam {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SELF_PARAM }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TypeBoundList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TYPE_BOUND_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RecordFieldList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RECORD_FIELD_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TupleFieldList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TUPLE_FIELD_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RecordField {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RECORD_FIELD }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TupleField {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TUPLE_FIELD }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for VariantList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == VARIANT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Variant {
    fn can_cast(kind: SyntaxKind) -> bool { kind == VARIANT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for AssocItemList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ASSOC_ITEM_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExternItemList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == EXTERN_ITEM_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for LifetimeParam {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LIFETIME_PARAM }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TypeParam {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TYPE_PARAM }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ConstParam {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CONST_PARAM }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ParenType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PAREN_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TupleType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TUPLE_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for NeverType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == NEVER_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PathType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PATH_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PointerType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == POINTER_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArrayType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ARRAY_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SliceType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SLICE_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ReferenceType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == REFERENCE_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PlaceholderType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PLACEHOLDER_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for FnPointerType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == FN_POINTER_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ForType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ImplTraitType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == IMPL_TRAIT_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DynTraitType {
    fn can_cast(kind: SyntaxKind) -> bool { kind == DYN_TRAIT_TYPE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TupleExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TUPLE_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArrayExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ARRAY_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ParenExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PAREN_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PathExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PATH_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for LambdaExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LAMBDA_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for IfExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == IF_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Condition {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CONDITION }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for EffectExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == EFFECT_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Label {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LABEL }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for LoopExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LOOP_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ForExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for WhileExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == WHILE_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ContinueExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CONTINUE_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BreakExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BREAK_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ReturnExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RETURN_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for CallExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CALL_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ArgList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ARG_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MethodCallExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == METHOD_CALL_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TypeArgList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TYPE_ARG_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for FieldExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == FIELD_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for IndexExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == INDEX_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for AwaitExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == AWAIT_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TryExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TRY_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for CastExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CAST_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RefExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == REF_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PrefixExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PREFIX_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BoxExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BOX_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RangeExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RANGE_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BinExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BIN_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Literal {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LITERAL }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MatchExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MATCH_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MatchArmList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MATCH_ARM_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MatchArm {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MATCH_ARM }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MatchGuard {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MATCH_GUARD }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RecordExpr {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RECORD_EXPR }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RecordExprFieldList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RECORD_EXPR_FIELD_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RecordExprField {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RECORD_EXPR_FIELD }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for OrPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == OR_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ParenPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PAREN_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RefPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == REF_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BoxPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BOX_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for BindPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BIND_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PlaceholderPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PLACEHOLDER_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for DotDotPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == DOT_DOT_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PathPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PATH_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for SlicePat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SLICE_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RangePat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RANGE_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for LiteralPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LITERAL_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MacroPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MACRO_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RecordPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RECORD_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RecordFieldPatList {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RECORD_FIELD_PAT_LIST }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for RecordFieldPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RECORD_FIELD_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TupleStructPat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TUPLE_STRUCT_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TuplePat {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TUPLE_PAT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TokenTree {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TOKEN_TREE }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MacroDef {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MACRO_DEF }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MacroItems {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MACRO_ITEMS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MacroStmts {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MACRO_STMTS }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TypeBound {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TYPE_BOUND }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for WherePred {
    fn can_cast(kind: SyntaxKind) -> bool { kind == WHERE_PRED }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ExprStmt {
    fn can_cast(kind: SyntaxKind) -> bool { kind == EXPR_STMT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for LetStmt {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LET_STMT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for PathSegment {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PATH_SEGMENT }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for TypeArg {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TYPE_ARG }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for LifetimeArg {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LIFETIME_ARG }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for AssocTypeArg {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ASSOC_TYPE_ARG }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for ConstArg {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CONST_ARG }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for MetaItem {
    fn can_cast(kind: SyntaxKind) -> bool { kind == META_ITEM }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl From<Const> for Item {
    fn from(node: Const) -> Item { Item::Const(node) }
}
impl From<Enum> for Item {
    fn from(node: Enum) -> Item { Item::Enum(node) }
}
impl From<ExternBlock> for Item {
    fn from(node: ExternBlock) -> Item { Item::ExternBlock(node) }
}
impl From<ExternCrate> for Item {
    fn from(node: ExternCrate) -> Item { Item::ExternCrate(node) }
}
impl From<Fn> for Item {
    fn from(node: Fn) -> Item { Item::Fn(node) }
}
impl From<Impl> for Item {
    fn from(node: Impl) -> Item { Item::Impl(node) }
}
impl From<MacroCall> for Item {
    fn from(node: MacroCall) -> Item { Item::MacroCall(node) }
}
impl From<Module> for Item {
    fn from(node: Module) -> Item { Item::Module(node) }
}
impl From<Static> for Item {
    fn from(node: Static) -> Item { Item::Static(node) }
}
impl From<Struct> for Item {
    fn from(node: Struct) -> Item { Item::Struct(node) }
}
impl From<Trait> for Item {
    fn from(node: Trait) -> Item { Item::Trait(node) }
}
impl From<TypeAlias> for Item {
    fn from(node: TypeAlias) -> Item { Item::TypeAlias(node) }
}
impl From<Union> for Item {
    fn from(node: Union) -> Item { Item::Union(node) }
}
impl From<Use> for Item {
    fn from(node: Use) -> Item { Item::Use(node) }
}
impl AstNode for Item {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            CONST | ENUM | EXTERN_BLOCK | EXTERN_CRATE | FN | IMPL | MACRO_CALL | MODULE
            | STATIC | STRUCT | TRAIT | TYPE_ALIAS | UNION | USE => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            CONST => Item::Const(Const { syntax }),
            ENUM => Item::Enum(Enum { syntax }),
            EXTERN_BLOCK => Item::ExternBlock(ExternBlock { syntax }),
            EXTERN_CRATE => Item::ExternCrate(ExternCrate { syntax }),
            FN => Item::Fn(Fn { syntax }),
            IMPL => Item::Impl(Impl { syntax }),
            MACRO_CALL => Item::MacroCall(MacroCall { syntax }),
            MODULE => Item::Module(Module { syntax }),
            STATIC => Item::Static(Static { syntax }),
            STRUCT => Item::Struct(Struct { syntax }),
            TRAIT => Item::Trait(Trait { syntax }),
            TYPE_ALIAS => Item::TypeAlias(TypeAlias { syntax }),
            UNION => Item::Union(Union { syntax }),
            USE => Item::Use(Use { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Item::Const(it) => &it.syntax,
            Item::Enum(it) => &it.syntax,
            Item::ExternBlock(it) => &it.syntax,
            Item::ExternCrate(it) => &it.syntax,
            Item::Fn(it) => &it.syntax,
            Item::Impl(it) => &it.syntax,
            Item::MacroCall(it) => &it.syntax,
            Item::Module(it) => &it.syntax,
            Item::Static(it) => &it.syntax,
            Item::Struct(it) => &it.syntax,
            Item::Trait(it) => &it.syntax,
            Item::TypeAlias(it) => &it.syntax,
            Item::Union(it) => &it.syntax,
            Item::Use(it) => &it.syntax,
        }
    }
}
impl From<ParenType> for TypeRef {
    fn from(node: ParenType) -> TypeRef { TypeRef::ParenType(node) }
}
impl From<TupleType> for TypeRef {
    fn from(node: TupleType) -> TypeRef { TypeRef::TupleType(node) }
}
impl From<NeverType> for TypeRef {
    fn from(node: NeverType) -> TypeRef { TypeRef::NeverType(node) }
}
impl From<PathType> for TypeRef {
    fn from(node: PathType) -> TypeRef { TypeRef::PathType(node) }
}
impl From<PointerType> for TypeRef {
    fn from(node: PointerType) -> TypeRef { TypeRef::PointerType(node) }
}
impl From<ArrayType> for TypeRef {
    fn from(node: ArrayType) -> TypeRef { TypeRef::ArrayType(node) }
}
impl From<SliceType> for TypeRef {
    fn from(node: SliceType) -> TypeRef { TypeRef::SliceType(node) }
}
impl From<ReferenceType> for TypeRef {
    fn from(node: ReferenceType) -> TypeRef { TypeRef::ReferenceType(node) }
}
impl From<PlaceholderType> for TypeRef {
    fn from(node: PlaceholderType) -> TypeRef { TypeRef::PlaceholderType(node) }
}
impl From<FnPointerType> for TypeRef {
    fn from(node: FnPointerType) -> TypeRef { TypeRef::FnPointerType(node) }
}
impl From<ForType> for TypeRef {
    fn from(node: ForType) -> TypeRef { TypeRef::ForType(node) }
}
impl From<ImplTraitType> for TypeRef {
    fn from(node: ImplTraitType) -> TypeRef { TypeRef::ImplTraitType(node) }
}
impl From<DynTraitType> for TypeRef {
    fn from(node: DynTraitType) -> TypeRef { TypeRef::DynTraitType(node) }
}
impl AstNode for TypeRef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PAREN_TYPE | TUPLE_TYPE | NEVER_TYPE | PATH_TYPE | POINTER_TYPE | ARRAY_TYPE
            | SLICE_TYPE | REFERENCE_TYPE | PLACEHOLDER_TYPE | FN_POINTER_TYPE | FOR_TYPE
            | IMPL_TRAIT_TYPE | DYN_TRAIT_TYPE => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PAREN_TYPE => TypeRef::ParenType(ParenType { syntax }),
            TUPLE_TYPE => TypeRef::TupleType(TupleType { syntax }),
            NEVER_TYPE => TypeRef::NeverType(NeverType { syntax }),
            PATH_TYPE => TypeRef::PathType(PathType { syntax }),
            POINTER_TYPE => TypeRef::PointerType(PointerType { syntax }),
            ARRAY_TYPE => TypeRef::ArrayType(ArrayType { syntax }),
            SLICE_TYPE => TypeRef::SliceType(SliceType { syntax }),
            REFERENCE_TYPE => TypeRef::ReferenceType(ReferenceType { syntax }),
            PLACEHOLDER_TYPE => TypeRef::PlaceholderType(PlaceholderType { syntax }),
            FN_POINTER_TYPE => TypeRef::FnPointerType(FnPointerType { syntax }),
            FOR_TYPE => TypeRef::ForType(ForType { syntax }),
            IMPL_TRAIT_TYPE => TypeRef::ImplTraitType(ImplTraitType { syntax }),
            DYN_TRAIT_TYPE => TypeRef::DynTraitType(DynTraitType { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            TypeRef::ParenType(it) => &it.syntax,
            TypeRef::TupleType(it) => &it.syntax,
            TypeRef::NeverType(it) => &it.syntax,
            TypeRef::PathType(it) => &it.syntax,
            TypeRef::PointerType(it) => &it.syntax,
            TypeRef::ArrayType(it) => &it.syntax,
            TypeRef::SliceType(it) => &it.syntax,
            TypeRef::ReferenceType(it) => &it.syntax,
            TypeRef::PlaceholderType(it) => &it.syntax,
            TypeRef::FnPointerType(it) => &it.syntax,
            TypeRef::ForType(it) => &it.syntax,
            TypeRef::ImplTraitType(it) => &it.syntax,
            TypeRef::DynTraitType(it) => &it.syntax,
        }
    }
}
impl From<OrPat> for Pat {
    fn from(node: OrPat) -> Pat { Pat::OrPat(node) }
}
impl From<ParenPat> for Pat {
    fn from(node: ParenPat) -> Pat { Pat::ParenPat(node) }
}
impl From<RefPat> for Pat {
    fn from(node: RefPat) -> Pat { Pat::RefPat(node) }
}
impl From<BoxPat> for Pat {
    fn from(node: BoxPat) -> Pat { Pat::BoxPat(node) }
}
impl From<BindPat> for Pat {
    fn from(node: BindPat) -> Pat { Pat::BindPat(node) }
}
impl From<PlaceholderPat> for Pat {
    fn from(node: PlaceholderPat) -> Pat { Pat::PlaceholderPat(node) }
}
impl From<DotDotPat> for Pat {
    fn from(node: DotDotPat) -> Pat { Pat::DotDotPat(node) }
}
impl From<PathPat> for Pat {
    fn from(node: PathPat) -> Pat { Pat::PathPat(node) }
}
impl From<RecordPat> for Pat {
    fn from(node: RecordPat) -> Pat { Pat::RecordPat(node) }
}
impl From<TupleStructPat> for Pat {
    fn from(node: TupleStructPat) -> Pat { Pat::TupleStructPat(node) }
}
impl From<TuplePat> for Pat {
    fn from(node: TuplePat) -> Pat { Pat::TuplePat(node) }
}
impl From<SlicePat> for Pat {
    fn from(node: SlicePat) -> Pat { Pat::SlicePat(node) }
}
impl From<RangePat> for Pat {
    fn from(node: RangePat) -> Pat { Pat::RangePat(node) }
}
impl From<LiteralPat> for Pat {
    fn from(node: LiteralPat) -> Pat { Pat::LiteralPat(node) }
}
impl From<MacroPat> for Pat {
    fn from(node: MacroPat) -> Pat { Pat::MacroPat(node) }
}
impl AstNode for Pat {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            OR_PAT | PAREN_PAT | REF_PAT | BOX_PAT | BIND_PAT | PLACEHOLDER_PAT | DOT_DOT_PAT
            | PATH_PAT | RECORD_PAT | TUPLE_STRUCT_PAT | TUPLE_PAT | SLICE_PAT | RANGE_PAT
            | LITERAL_PAT | MACRO_PAT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            OR_PAT => Pat::OrPat(OrPat { syntax }),
            PAREN_PAT => Pat::ParenPat(ParenPat { syntax }),
            REF_PAT => Pat::RefPat(RefPat { syntax }),
            BOX_PAT => Pat::BoxPat(BoxPat { syntax }),
            BIND_PAT => Pat::BindPat(BindPat { syntax }),
            PLACEHOLDER_PAT => Pat::PlaceholderPat(PlaceholderPat { syntax }),
            DOT_DOT_PAT => Pat::DotDotPat(DotDotPat { syntax }),
            PATH_PAT => Pat::PathPat(PathPat { syntax }),
            RECORD_PAT => Pat::RecordPat(RecordPat { syntax }),
            TUPLE_STRUCT_PAT => Pat::TupleStructPat(TupleStructPat { syntax }),
            TUPLE_PAT => Pat::TuplePat(TuplePat { syntax }),
            SLICE_PAT => Pat::SlicePat(SlicePat { syntax }),
            RANGE_PAT => Pat::RangePat(RangePat { syntax }),
            LITERAL_PAT => Pat::LiteralPat(LiteralPat { syntax }),
            MACRO_PAT => Pat::MacroPat(MacroPat { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Pat::OrPat(it) => &it.syntax,
            Pat::ParenPat(it) => &it.syntax,
            Pat::RefPat(it) => &it.syntax,
            Pat::BoxPat(it) => &it.syntax,
            Pat::BindPat(it) => &it.syntax,
            Pat::PlaceholderPat(it) => &it.syntax,
            Pat::DotDotPat(it) => &it.syntax,
            Pat::PathPat(it) => &it.syntax,
            Pat::RecordPat(it) => &it.syntax,
            Pat::TupleStructPat(it) => &it.syntax,
            Pat::TuplePat(it) => &it.syntax,
            Pat::SlicePat(it) => &it.syntax,
            Pat::RangePat(it) => &it.syntax,
            Pat::LiteralPat(it) => &it.syntax,
            Pat::MacroPat(it) => &it.syntax,
        }
    }
}
impl From<RecordFieldList> for FieldList {
    fn from(node: RecordFieldList) -> FieldList { FieldList::RecordFieldList(node) }
}
impl From<TupleFieldList> for FieldList {
    fn from(node: TupleFieldList) -> FieldList { FieldList::TupleFieldList(node) }
}
impl AstNode for FieldList {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            RECORD_FIELD_LIST | TUPLE_FIELD_LIST => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            RECORD_FIELD_LIST => FieldList::RecordFieldList(RecordFieldList { syntax }),
            TUPLE_FIELD_LIST => FieldList::TupleFieldList(TupleFieldList { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            FieldList::RecordFieldList(it) => &it.syntax,
            FieldList::TupleFieldList(it) => &it.syntax,
        }
    }
}
impl From<TupleExpr> for Expr {
    fn from(node: TupleExpr) -> Expr { Expr::TupleExpr(node) }
}
impl From<ArrayExpr> for Expr {
    fn from(node: ArrayExpr) -> Expr { Expr::ArrayExpr(node) }
}
impl From<ParenExpr> for Expr {
    fn from(node: ParenExpr) -> Expr { Expr::ParenExpr(node) }
}
impl From<PathExpr> for Expr {
    fn from(node: PathExpr) -> Expr { Expr::PathExpr(node) }
}
impl From<LambdaExpr> for Expr {
    fn from(node: LambdaExpr) -> Expr { Expr::LambdaExpr(node) }
}
impl From<IfExpr> for Expr {
    fn from(node: IfExpr) -> Expr { Expr::IfExpr(node) }
}
impl From<LoopExpr> for Expr {
    fn from(node: LoopExpr) -> Expr { Expr::LoopExpr(node) }
}
impl From<ForExpr> for Expr {
    fn from(node: ForExpr) -> Expr { Expr::ForExpr(node) }
}
impl From<WhileExpr> for Expr {
    fn from(node: WhileExpr) -> Expr { Expr::WhileExpr(node) }
}
impl From<ContinueExpr> for Expr {
    fn from(node: ContinueExpr) -> Expr { Expr::ContinueExpr(node) }
}
impl From<BreakExpr> for Expr {
    fn from(node: BreakExpr) -> Expr { Expr::BreakExpr(node) }
}
impl From<Label> for Expr {
    fn from(node: Label) -> Expr { Expr::Label(node) }
}
impl From<BlockExpr> for Expr {
    fn from(node: BlockExpr) -> Expr { Expr::BlockExpr(node) }
}
impl From<ReturnExpr> for Expr {
    fn from(node: ReturnExpr) -> Expr { Expr::ReturnExpr(node) }
}
impl From<MatchExpr> for Expr {
    fn from(node: MatchExpr) -> Expr { Expr::MatchExpr(node) }
}
impl From<RecordExpr> for Expr {
    fn from(node: RecordExpr) -> Expr { Expr::RecordExpr(node) }
}
impl From<CallExpr> for Expr {
    fn from(node: CallExpr) -> Expr { Expr::CallExpr(node) }
}
impl From<IndexExpr> for Expr {
    fn from(node: IndexExpr) -> Expr { Expr::IndexExpr(node) }
}
impl From<MethodCallExpr> for Expr {
    fn from(node: MethodCallExpr) -> Expr { Expr::MethodCallExpr(node) }
}
impl From<FieldExpr> for Expr {
    fn from(node: FieldExpr) -> Expr { Expr::FieldExpr(node) }
}
impl From<AwaitExpr> for Expr {
    fn from(node: AwaitExpr) -> Expr { Expr::AwaitExpr(node) }
}
impl From<TryExpr> for Expr {
    fn from(node: TryExpr) -> Expr { Expr::TryExpr(node) }
}
impl From<EffectExpr> for Expr {
    fn from(node: EffectExpr) -> Expr { Expr::EffectExpr(node) }
}
impl From<CastExpr> for Expr {
    fn from(node: CastExpr) -> Expr { Expr::CastExpr(node) }
}
impl From<RefExpr> for Expr {
    fn from(node: RefExpr) -> Expr { Expr::RefExpr(node) }
}
impl From<PrefixExpr> for Expr {
    fn from(node: PrefixExpr) -> Expr { Expr::PrefixExpr(node) }
}
impl From<RangeExpr> for Expr {
    fn from(node: RangeExpr) -> Expr { Expr::RangeExpr(node) }
}
impl From<BinExpr> for Expr {
    fn from(node: BinExpr) -> Expr { Expr::BinExpr(node) }
}
impl From<Literal> for Expr {
    fn from(node: Literal) -> Expr { Expr::Literal(node) }
}
impl From<MacroCall> for Expr {
    fn from(node: MacroCall) -> Expr { Expr::MacroCall(node) }
}
impl From<BoxExpr> for Expr {
    fn from(node: BoxExpr) -> Expr { Expr::BoxExpr(node) }
}
impl AstNode for Expr {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            TUPLE_EXPR | ARRAY_EXPR | PAREN_EXPR | PATH_EXPR | LAMBDA_EXPR | IF_EXPR
            | LOOP_EXPR | FOR_EXPR | WHILE_EXPR | CONTINUE_EXPR | BREAK_EXPR | LABEL
            | BLOCK_EXPR | RETURN_EXPR | MATCH_EXPR | RECORD_EXPR | CALL_EXPR | INDEX_EXPR
            | METHOD_CALL_EXPR | FIELD_EXPR | AWAIT_EXPR | TRY_EXPR | EFFECT_EXPR | CAST_EXPR
            | REF_EXPR | PREFIX_EXPR | RANGE_EXPR | BIN_EXPR | LITERAL | MACRO_CALL | BOX_EXPR => {
                true
            }
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            TUPLE_EXPR => Expr::TupleExpr(TupleExpr { syntax }),
            ARRAY_EXPR => Expr::ArrayExpr(ArrayExpr { syntax }),
            PAREN_EXPR => Expr::ParenExpr(ParenExpr { syntax }),
            PATH_EXPR => Expr::PathExpr(PathExpr { syntax }),
            LAMBDA_EXPR => Expr::LambdaExpr(LambdaExpr { syntax }),
            IF_EXPR => Expr::IfExpr(IfExpr { syntax }),
            LOOP_EXPR => Expr::LoopExpr(LoopExpr { syntax }),
            FOR_EXPR => Expr::ForExpr(ForExpr { syntax }),
            WHILE_EXPR => Expr::WhileExpr(WhileExpr { syntax }),
            CONTINUE_EXPR => Expr::ContinueExpr(ContinueExpr { syntax }),
            BREAK_EXPR => Expr::BreakExpr(BreakExpr { syntax }),
            LABEL => Expr::Label(Label { syntax }),
            BLOCK_EXPR => Expr::BlockExpr(BlockExpr { syntax }),
            RETURN_EXPR => Expr::ReturnExpr(ReturnExpr { syntax }),
            MATCH_EXPR => Expr::MatchExpr(MatchExpr { syntax }),
            RECORD_EXPR => Expr::RecordExpr(RecordExpr { syntax }),
            CALL_EXPR => Expr::CallExpr(CallExpr { syntax }),
            INDEX_EXPR => Expr::IndexExpr(IndexExpr { syntax }),
            METHOD_CALL_EXPR => Expr::MethodCallExpr(MethodCallExpr { syntax }),
            FIELD_EXPR => Expr::FieldExpr(FieldExpr { syntax }),
            AWAIT_EXPR => Expr::AwaitExpr(AwaitExpr { syntax }),
            TRY_EXPR => Expr::TryExpr(TryExpr { syntax }),
            EFFECT_EXPR => Expr::EffectExpr(EffectExpr { syntax }),
            CAST_EXPR => Expr::CastExpr(CastExpr { syntax }),
            REF_EXPR => Expr::RefExpr(RefExpr { syntax }),
            PREFIX_EXPR => Expr::PrefixExpr(PrefixExpr { syntax }),
            RANGE_EXPR => Expr::RangeExpr(RangeExpr { syntax }),
            BIN_EXPR => Expr::BinExpr(BinExpr { syntax }),
            LITERAL => Expr::Literal(Literal { syntax }),
            MACRO_CALL => Expr::MacroCall(MacroCall { syntax }),
            BOX_EXPR => Expr::BoxExpr(BoxExpr { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Expr::TupleExpr(it) => &it.syntax,
            Expr::ArrayExpr(it) => &it.syntax,
            Expr::ParenExpr(it) => &it.syntax,
            Expr::PathExpr(it) => &it.syntax,
            Expr::LambdaExpr(it) => &it.syntax,
            Expr::IfExpr(it) => &it.syntax,
            Expr::LoopExpr(it) => &it.syntax,
            Expr::ForExpr(it) => &it.syntax,
            Expr::WhileExpr(it) => &it.syntax,
            Expr::ContinueExpr(it) => &it.syntax,
            Expr::BreakExpr(it) => &it.syntax,
            Expr::Label(it) => &it.syntax,
            Expr::BlockExpr(it) => &it.syntax,
            Expr::ReturnExpr(it) => &it.syntax,
            Expr::MatchExpr(it) => &it.syntax,
            Expr::RecordExpr(it) => &it.syntax,
            Expr::CallExpr(it) => &it.syntax,
            Expr::IndexExpr(it) => &it.syntax,
            Expr::MethodCallExpr(it) => &it.syntax,
            Expr::FieldExpr(it) => &it.syntax,
            Expr::AwaitExpr(it) => &it.syntax,
            Expr::TryExpr(it) => &it.syntax,
            Expr::EffectExpr(it) => &it.syntax,
            Expr::CastExpr(it) => &it.syntax,
            Expr::RefExpr(it) => &it.syntax,
            Expr::PrefixExpr(it) => &it.syntax,
            Expr::RangeExpr(it) => &it.syntax,
            Expr::BinExpr(it) => &it.syntax,
            Expr::Literal(it) => &it.syntax,
            Expr::MacroCall(it) => &it.syntax,
            Expr::BoxExpr(it) => &it.syntax,
        }
    }
}
impl From<Fn> for AssocItem {
    fn from(node: Fn) -> AssocItem { AssocItem::Fn(node) }
}
impl From<TypeAlias> for AssocItem {
    fn from(node: TypeAlias) -> AssocItem { AssocItem::TypeAlias(node) }
}
impl From<Const> for AssocItem {
    fn from(node: Const) -> AssocItem { AssocItem::Const(node) }
}
impl From<MacroCall> for AssocItem {
    fn from(node: MacroCall) -> AssocItem { AssocItem::MacroCall(node) }
}
impl AstNode for AssocItem {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FN | TYPE_ALIAS | CONST | MACRO_CALL => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            FN => AssocItem::Fn(Fn { syntax }),
            TYPE_ALIAS => AssocItem::TypeAlias(TypeAlias { syntax }),
            CONST => AssocItem::Const(Const { syntax }),
            MACRO_CALL => AssocItem::MacroCall(MacroCall { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AssocItem::Fn(it) => &it.syntax,
            AssocItem::TypeAlias(it) => &it.syntax,
            AssocItem::Const(it) => &it.syntax,
            AssocItem::MacroCall(it) => &it.syntax,
        }
    }
}
impl From<Fn> for ExternItem {
    fn from(node: Fn) -> ExternItem { ExternItem::Fn(node) }
}
impl From<Static> for ExternItem {
    fn from(node: Static) -> ExternItem { ExternItem::Static(node) }
}
impl From<MacroCall> for ExternItem {
    fn from(node: MacroCall) -> ExternItem { ExternItem::MacroCall(node) }
}
impl AstNode for ExternItem {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            FN | STATIC | MACRO_CALL => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            FN => ExternItem::Fn(Fn { syntax }),
            STATIC => ExternItem::Static(Static { syntax }),
            MACRO_CALL => ExternItem::MacroCall(MacroCall { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            ExternItem::Fn(it) => &it.syntax,
            ExternItem::Static(it) => &it.syntax,
            ExternItem::MacroCall(it) => &it.syntax,
        }
    }
}
impl From<LifetimeParam> for GenericParam {
    fn from(node: LifetimeParam) -> GenericParam { GenericParam::LifetimeParam(node) }
}
impl From<TypeParam> for GenericParam {
    fn from(node: TypeParam) -> GenericParam { GenericParam::TypeParam(node) }
}
impl From<ConstParam> for GenericParam {
    fn from(node: ConstParam) -> GenericParam { GenericParam::ConstParam(node) }
}
impl AstNode for GenericParam {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LIFETIME_PARAM | TYPE_PARAM | CONST_PARAM => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            LIFETIME_PARAM => GenericParam::LifetimeParam(LifetimeParam { syntax }),
            TYPE_PARAM => GenericParam::TypeParam(TypeParam { syntax }),
            CONST_PARAM => GenericParam::ConstParam(ConstParam { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            GenericParam::LifetimeParam(it) => &it.syntax,
            GenericParam::TypeParam(it) => &it.syntax,
            GenericParam::ConstParam(it) => &it.syntax,
        }
    }
}
impl From<LetStmt> for Stmt {
    fn from(node: LetStmt) -> Stmt { Stmt::LetStmt(node) }
}
impl From<ExprStmt> for Stmt {
    fn from(node: ExprStmt) -> Stmt { Stmt::ExprStmt(node) }
}
impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LET_STMT | EXPR_STMT => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            LET_STMT => Stmt::LetStmt(LetStmt { syntax }),
            EXPR_STMT => Stmt::ExprStmt(ExprStmt { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Stmt::LetStmt(it) => &it.syntax,
            Stmt::ExprStmt(it) => &it.syntax,
        }
    }
}
impl From<Literal> for AttrInput {
    fn from(node: Literal) -> AttrInput { AttrInput::Literal(node) }
}
impl From<TokenTree> for AttrInput {
    fn from(node: TokenTree) -> AttrInput { AttrInput::TokenTree(node) }
}
impl AstNode for AttrInput {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            LITERAL | TOKEN_TREE => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            LITERAL => AttrInput::Literal(Literal { syntax }),
            TOKEN_TREE => AttrInput::TokenTree(TokenTree { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AttrInput::Literal(it) => &it.syntax,
            AttrInput::TokenTree(it) => &it.syntax,
        }
    }
}
impl From<Struct> for AdtDef {
    fn from(node: Struct) -> AdtDef { AdtDef::Struct(node) }
}
impl From<Enum> for AdtDef {
    fn from(node: Enum) -> AdtDef { AdtDef::Enum(node) }
}
impl From<Union> for AdtDef {
    fn from(node: Union) -> AdtDef { AdtDef::Union(node) }
}
impl AstNode for AdtDef {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            STRUCT | ENUM | UNION => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            STRUCT => AdtDef::Struct(Struct { syntax }),
            ENUM => AdtDef::Enum(Enum { syntax }),
            UNION => AdtDef::Union(Union { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            AdtDef::Struct(it) => &it.syntax,
            AdtDef::Enum(it) => &it.syntax,
            AdtDef::Union(it) => &it.syntax,
        }
    }
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Pat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AssocItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExternItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GenericParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AttrInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AdtDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SourceFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Attr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Const {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExternBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExternCrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Fn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Impl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MacroCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Static {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Struct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Trait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Union {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Use {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NameRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Rename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for UseTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for UseTreeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Abi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for GenericParamList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParamList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for WhereClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BlockExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Param {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SelfParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeBoundList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecordFieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TupleFieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecordField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TupleField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for VariantList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AssocItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExternItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LifetimeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ConstParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TupleType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for NeverType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PathType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ArrayType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SliceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ReferenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PlaceholderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FnPointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ForType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ImplTraitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DynTraitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TupleExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ArrayExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParenExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PathExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LambdaExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IfExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for EffectExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LoopExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ForExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for WhileExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ContinueExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BreakExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ReturnExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ArgList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MethodCallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeArgList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for FieldExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for IndexExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AwaitExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for CastExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RefExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PrefixExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BoxExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RangeExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BinExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MatchExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MatchArmList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MatchArm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MatchGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecordExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecordExprFieldList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecordExprField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for OrPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ParenPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RefPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BoxPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for BindPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PlaceholderPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for DotDotPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PathPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SlicePat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RangePat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LiteralPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MacroPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecordPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecordFieldPatList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for RecordFieldPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TupleStructPat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TuplePat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TokenTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MacroDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MacroItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MacroStmts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeBound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for WherePred {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LetStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for TypeArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for LifetimeArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AssocTypeArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for ConstArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MetaItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
