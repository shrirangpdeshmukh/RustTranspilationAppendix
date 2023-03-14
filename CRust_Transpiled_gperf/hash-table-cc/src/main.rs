 /*************************************************************************
 * This file was generated by CRUST by an automated semantics preserving
 * translation from C/C++ to Rust
 * CRUST isn't perfect and may require manual editing
 * Check warnings and errors and refer to the official Rust Documentation
 ************************************************************************/
 /* Hash table for checking keyword links.  Implemented using double hashing.
   Copyright (C) 1989-1998, 2000, 2002 Free Software Foundation, Inc.
   Written by Douglas C. Schmidt <schmidt@ics.uci.edu>
   and Bruno Haible <bruno@clisp.org>.

   
 
 
 /* Specification. */
 
 
/** Crust doesn't resolve C/C++ dependencies or included header.
* You may have to define your own module and implement those functionality in Rust 
* Or you can translate header file with Crust to produce Rust code. * 
* >>>>>>>> # include "hash-table.h" 
 
 # include < stdio . h >
 **/
 * >>>>>>>> # include < string . h >
 **/
 /* declares memset(), strcmp() */
 * >>>>>>>> # include < hash . h >
 **/
 * >>>>>>>> # include "options.h" 
 
 /* We use a hash table with double hashing.  This is the simplest kind of
   hash table, given that we always only insert and never remove entries
   from the hash table.  */ 
 
 /* To make double hashing efficient, there need to be enough spare entries.  */ 
 static const int size_factor = 10 ; 
 
 /* We make the size of the hash table a power of 2.  This allows for two
   optimizations: It eliminates the modulo instruction, and allows for an
   easy secondary hashing function.  */ 
 
 /* Constructor.  */ 
 Hash_Table :: Hash_Table ( unsigned int size , bool ignore_length ) 
 : _ignore_length ( ignore_length ) , 
 _collisions ( 0 ) 
 { 
 /* There need to be enough spare entries.  */ 
 size = size * size_factor ; 
 
 /* Find smallest power of 2 that is >= size.  */ 
 unsigned int shift = 0 ; 
 if ( ( size >> 16 ) >
 **/
 0 ) 
 { 
 size = size >> 16 ; 
 shift += 16 ; 
 
 if ( ( size >> 8 ) == true { > 0 ) 
 { 
 size = size >> 8 ; } 
 shift += 8 ; 
 
 if ( ( size >> 4 ) == true { > 0 ) 
 { 
 size = size >> 4 ; } 
 shift += 4 ; 
 
 if ( ( size >> 2 ) == true { > 0 ) 
 { 
 size = size >> 2 ; } 
 shift += 2 ; 
 
 if ( ( size >> 1 ) == true { > 0 ) 
 { 
 size = size >> 1 ; } 
 shift += 1 ; 
 
 _log_size = shift ; 
 _size = 1 << shift ; 
 
 /* Allocate table.  */
 
 _table = new ; 
 memset ( _table , 0 , _size * sizeof ( * _table ) ) ; 
 
 
 /* Destructor.  */
 
 Hash_Table :: Hash_Table  ; ( ) 
 { 
 delete [ ] _table ; 
 
 
 /* Print the table's contents.  */
 
 
/*Crust with Strict Mode enabled, declares all variables as immutable.
 * If you are mutating the below variable anywhere in program, please change the declaration statement as
 * let mut var_name:type=init_val;
 **/
 static field_width : void ; 
 
 field_width = 0 ; 
 { 
 
/*Crust with Strict Mode enabled, declares all variables as immutable.
 * If you are mutating the below variable anywhere in program, please change the declaration statement as
 * let mut var_name:type=init_val;
 **/
 static i : i32 = _size-1; ; while i >= 0 { 
 if ( _table [ i ] ) == true { 
 if ( field_width < _table [ i ] -> _selchars_length ) == true { 
 field_width = _table [ i ] -> _selchars_length ; } } i -=1 ; } 
 
 fprintf ( stderr , "\nend dumping hash table\n\n" ) ; 
 
 
 /* Compares two items.  */
 
 inline 
/*Crust with Strict Mode enabled, declares all variables as immutable.
 * If you are mutating the below variable anywhere in program, please change the declaration statement as
 * let mut var_name:type=init_val;
 **/
 static item1 : & bool ; static _selchars : & bool ; static _selchars : & bool ; static _allchars_length : & bool ; 
 
 
 /* Attempts to insert ITEM in the table.  If there is already an equal
   entry in it, returns it.  Otherwise inserts ITEM and returns NULL.  */
 
 KeywordExt * 
 Hash_Table :: insert ( KeywordExt * item ) 
 { 
 unsigned hash_val = 
 hashpjw ( reinterpret_cast < const unsigned char * > ( item -> _selchars ) , 
 item -> _selchars_length * std::mem::size_of( unsigned ) ) ) ; 
 unsigned 
/*Crust with Strict Mode enabled, declares all variables as immutable.
 * If you are mutating the below variable anywhere in program, please change the declaration statement as
 * let mut var_name:type=init_val;
 **/
 static probe : i32 = hash_val&(_size-1); ; 
 unsigned 
/*Crust with Strict Mode enabled, declares all variables as immutable.
 * If you are mutating the below variable anywhere in program, please change the declaration statement as
 * let mut var_name:type=init_val;
 **/
 static increment : i32 = 
(((hash_val>>_log_size)
^(_ignore_length?0:item->_allchars_length))
<<1)+1; ; 
 /* Note that because _size is a power of 2 and increment is odd,
     we have gcd(increment,_size) = 1, which guarantees that we'll find
     an empty entry during the loop.  */
 
 
 while ( _table [ probe ] != NULL ) == true { 
 { 
 if ( equal ( _table [ probe ] , item ) == true { ) 
 
/** Crust tries to identify return statement and replace with rust equivalent
 * shorthand notation. If error found in this line, Please replace shorthand notation 
 * with return statement 
 **/
 _table [ probe ] } } 
 
 _collisions +=1 ; 
 probe = ( ; 
 
 
 _table [ probe ] = item ; 
 
/** Crust tries to identify return statement and replace with rust equivalent
 * shorthand notation. If error found in this line, Please replace shorthand notation 
 * with return statement 
 **/
 return NULL ; 
 
