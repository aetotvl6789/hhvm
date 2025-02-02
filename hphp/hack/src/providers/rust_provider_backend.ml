(*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree.
 *
 *)

open Hh_prelude

type t

external make_ffi :
  root:string -> hhi_root:string -> tmp:string -> ParserOptions.t -> t
  = "hh_rust_provider_backend_make"

let make popt =
  make_ffi
    ~root:Relative_path.(path_of_prefix Root)
    ~hhi_root:Relative_path.(path_of_prefix Hhi)
    ~tmp:Relative_path.(path_of_prefix Tmp)
    popt

module Decl = struct
  external direct_decl_parse_and_cache :
    t -> Relative_path.t -> Direct_decl_parser.parsed_file_with_hashes option
    = "hh_rust_provider_backend_direct_decl_parse_and_cache"

  external get_fun : t -> string -> Shallow_decl_defs.fun_decl option
    = "hh_rust_provider_backend_get_fun"

  external get_shallow_class :
    t -> string -> Shallow_decl_defs.class_decl option
    = "hh_rust_provider_backend_get_shallow_class"

  external get_typedef : t -> string -> Shallow_decl_defs.typedef_decl option
    = "hh_rust_provider_backend_get_typedef"

  external get_gconst : t -> string -> Shallow_decl_defs.const_decl option
    = "hh_rust_provider_backend_get_gconst"

  external get_module : t -> string -> Shallow_decl_defs.module_decl option
    = "hh_rust_provider_backend_get_module"

  external get_folded_class : t -> string -> Decl_defs.decl_class_type option
    = "hh_rust_provider_backend_get_folded_class"

  external push_local_changes : t -> unit
    = "hh_rust_provider_backend_decl_provider_push_local_changes"

  external pop_local_changes : t -> unit
    = "hh_rust_provider_backend_decl_provider_pop_local_changes"
end

module File = struct
  type file_type =
    | Disk of string
    | Ide of string

  external get : t -> Relative_path.t -> file_type option
    = "hh_rust_provider_backend_file_provider_get"

  external get_contents : t -> Relative_path.t -> string option
    = "hh_rust_provider_backend_file_provider_get_contents"

  external provide_file_for_tests : t -> Relative_path.t -> string -> unit
    = "hh_rust_provider_backend_file_provider_provide_file_for_tests"

  external provide_file_for_ide : t -> Relative_path.t -> string -> unit
    = "hh_rust_provider_backend_file_provider_provide_file_for_ide"

  external provide_file_hint : t -> Relative_path.t -> file_type -> unit
    = "hh_rust_provider_backend_file_provider_provide_file_hint"

  external remove_batch : t -> Relative_path.Set.t -> unit
    = "hh_rust_provider_backend_file_provider_remove_batch"

  external push_local_changes : t -> unit
    = "hh_rust_provider_backend_file_provider_push_local_changes"

  external pop_local_changes : t -> unit
    = "hh_rust_provider_backend_file_provider_pop_local_changes"
end

module Naming = struct
  module type ReverseNamingTable = sig
    type pos

    val add : t -> string -> pos -> unit

    val get_pos : t -> Naming_sqlite.db_path option -> string -> pos option

    val remove_batch : t -> Naming_sqlite.db_path option -> string list -> unit
  end

  let unwrap_db_path (Naming_sqlite.Db_path path) = path

  let unwrap_db_path_opt = Option.map ~f:unwrap_db_path

  module Types = struct
    type pos = FileInfo.pos * Naming_types.kind_of_type

    external add : t -> string -> pos -> unit
      = "hh_rust_provider_backend_naming_types_add"

    external get_pos_ffi : t -> string option -> string -> pos option
      = "hh_rust_provider_backend_naming_types_get_pos"

    external remove_batch_ffi : t -> string option -> string list -> unit
      = "hh_rust_provider_backend_naming_types_remove_batch"

    external get_canon_name_ffi : t -> string option -> string -> string option
      = "hh_rust_provider_backend_naming_types_get_canon_name"

    let get_pos t path name = get_pos_ffi t (unwrap_db_path_opt path) name

    let remove_batch t path names =
      remove_batch_ffi t (unwrap_db_path_opt path) names

    let get_canon_name t path name =
      get_canon_name_ffi t (unwrap_db_path_opt path) name
  end

  module Funs = struct
    type pos = FileInfo.pos

    external add : t -> string -> pos -> unit
      = "hh_rust_provider_backend_naming_funs_add"

    external get_pos_ffi : t -> string option -> string -> pos option
      = "hh_rust_provider_backend_naming_funs_get_pos"

    external remove_batch_ffi : t -> string option -> string list -> unit
      = "hh_rust_provider_backend_naming_funs_remove_batch"

    external get_canon_name_ffi : t -> string option -> string -> string option
      = "hh_rust_provider_backend_naming_funs_get_canon_name"

    let get_pos t path name = get_pos_ffi t (unwrap_db_path_opt path) name

    let remove_batch t path names =
      remove_batch_ffi t (unwrap_db_path_opt path) names

    let get_canon_name t path name =
      get_canon_name_ffi t (unwrap_db_path_opt path) name
  end

  module Consts = struct
    type pos = FileInfo.pos

    external add : t -> string -> pos -> unit
      = "hh_rust_provider_backend_naming_consts_add"

    external get_pos_ffi : t -> string option -> string -> pos option
      = "hh_rust_provider_backend_naming_consts_get_pos"

    external remove_batch_ffi : t -> string option -> string list -> unit
      = "hh_rust_provider_backend_naming_consts_remove_batch"

    let get_pos t path name = get_pos_ffi t (unwrap_db_path_opt path) name

    let remove_batch t path names =
      remove_batch_ffi t (unwrap_db_path_opt path) names
  end

  module Modules = struct
    type pos = FileInfo.pos

    external add : t -> string -> pos -> unit
      = "hh_rust_provider_backend_naming_modules_add"

    external get_pos_ffi : t -> string option -> string -> pos option
      = "hh_rust_provider_backend_naming_modules_get_pos"

    external remove_batch_ffi : t -> string option -> string list -> unit
      = "hh_rust_provider_backend_naming_modules_remove_batch"

    let get_pos t path name = get_pos_ffi t (unwrap_db_path_opt path) name

    let remove_batch t path names =
      remove_batch_ffi t (unwrap_db_path_opt path) names
  end

  external get_filenames_by_hash_ffi :
    t -> string option -> Typing_deps.DepSet.t -> Relative_path.Set.t
    = "hh_rust_provider_backend_naming_get_filenames_by_hash"

  let get_filenames_by_hash t path deps =
    get_filenames_by_hash_ffi t (unwrap_db_path_opt path) deps

  external push_local_changes : t -> unit
    = "hh_rust_provider_backend_naming_push_local_changes"

  external pop_local_changes : t -> unit
    = "hh_rust_provider_backend_naming_pop_local_changes"
end
