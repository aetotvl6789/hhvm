(*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)

type t

val make : ParserOptions.t -> t

module Decl : sig
  val direct_decl_parse_and_cache :
    t -> Relative_path.t -> Direct_decl_parser.parsed_file_with_hashes option

  val get_fun : t -> string -> Shallow_decl_defs.fun_decl option

  val get_shallow_class : t -> string -> Shallow_decl_defs.class_decl option

  val get_typedef : t -> string -> Shallow_decl_defs.typedef_decl option

  val get_gconst : t -> string -> Shallow_decl_defs.const_decl option

  val get_module : t -> string -> Shallow_decl_defs.module_decl option

  val get_folded_class : t -> string -> Decl_defs.decl_class_type option

  val push_local_changes : t -> unit

  val pop_local_changes : t -> unit
end

module File : sig
  type file_type =
    | Disk of string
    | Ide of string

  val get : t -> Relative_path.t -> file_type option

  val get_contents : t -> Relative_path.t -> string option

  val provide_file_for_tests : t -> Relative_path.t -> string -> unit

  val provide_file_for_ide : t -> Relative_path.t -> string -> unit

  val provide_file_hint : t -> Relative_path.t -> file_type -> unit

  val remove_batch : t -> Relative_path.Set.t -> unit

  val push_local_changes : t -> unit

  val pop_local_changes : t -> unit
end

module Naming : sig
  module type ReverseNamingTable = sig
    type pos

    val add : t -> string -> pos -> unit

    val get_pos : t -> Naming_sqlite.db_path option -> string -> pos option

    val remove_batch : t -> Naming_sqlite.db_path option -> string list -> unit
  end

  module Types : sig
    include
      ReverseNamingTable
        with type pos = FileInfo.pos * Naming_types.kind_of_type

    val get_canon_name :
      t -> Naming_sqlite.db_path option -> string -> string option
  end

  module Funs : sig
    include ReverseNamingTable with type pos = FileInfo.pos

    val get_canon_name :
      t -> Naming_sqlite.db_path option -> string -> string option
  end

  module Consts : sig
    include ReverseNamingTable with type pos = FileInfo.pos
  end

  module Modules : sig
    include ReverseNamingTable with type pos = FileInfo.pos
  end

  (** This function searches all three namespaces (types, funs, consts) to
      find which one contains each Dep.t. The earlier functions in this module
      only search one specified namespace. Note: this function doesn't
      use the sharedmem cache of names - doesn't benefit from it, doesn't
      write into it. *)
  val get_filenames_by_hash :
    t ->
    Naming_sqlite.db_path option ->
    Typing_deps.DepSet.t ->
    Relative_path.Set.t

  val push_local_changes : t -> unit

  val pop_local_changes : t -> unit
end
