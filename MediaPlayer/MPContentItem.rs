//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use crate::common::*;
use crate::AppKit::*;
use crate::Foundation::*;
use crate::MediaPlayer::*;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    #[cfg(feature = "MediaPlayer_MPContentItem")]
    /**
      MPContentItem represents high-level metadata for a particular media item for
     representation outside the client application. Examples of media items that a
     developer might want to represent include song files, streaming audio URLs,
     or radio stations.
    */
    pub struct MPContentItem;

    #[cfg(feature = "MediaPlayer_MPContentItem")]
    unsafe impl ClassType for MPContentItem {
        type Super = NSObject;
    }
);

#[cfg(feature = "MediaPlayer_MPContentItem")]
/**
  MPContentItem represents high-level metadata for a particular media item for
 representation outside the client application. Examples of media items that a
 developer might want to represent include song files, streaming audio URLs,
 or radio stations.
*/
unsafe impl NSObjectProtocol for MPContentItem {}

extern_methods!(
    /**
      MPContentItem represents high-level metadata for a particular media item for
     representation outside the client application. Examples of media items that a
     developer might want to represent include song files, streaming audio URLs,
     or radio stations.
    */
    #[cfg(feature = "MediaPlayer_MPContentItem")]
    unsafe impl MPContentItem {
        #[cfg(feature = "Foundation_NSString")]
        #[method_id(@__retain_semantics Init initWithIdentifier:)]
        pub unsafe fn initWithIdentifier(
            this: Option<Allocated<Self>>,
            identifier: &NSString,
        ) -> Id<Self>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          A unique identifier for this content item. (Required)
        */
        #[method_id(@__retain_semantics Other identifier)]
        pub unsafe fn identifier(&self) -> Id<NSString>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          A title for this item. Usually this would be the track name, if representing
         a song, the episode name of a podcast, etc.
        */
        #[method_id(@__retain_semantics Other title)]
        pub unsafe fn title(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          A title for this item. Usually this would be the track name, if representing
         a song, the episode name of a podcast, etc.
        */
        #[method(setTitle:)]
        pub unsafe fn setTitle(&self, title: Option<&NSString>);

        #[cfg(feature = "Foundation_NSString")]
        /**
          A subtitle for this item. If this were representing a song, this would
         usually be the artist or composer.
        */
        #[method_id(@__retain_semantics Other subtitle)]
        pub unsafe fn subtitle(&self) -> Option<Id<NSString>>;

        #[cfg(feature = "Foundation_NSString")]
        /**
          A subtitle for this item. If this were representing a song, this would
         usually be the artist or composer.
        */
        #[method(setSubtitle:)]
        pub unsafe fn setSubtitle(&self, subtitle: Option<&NSString>);

        #[cfg(feature = "MediaPlayer_MPMediaItemArtwork")]
        /**
          Artwork for this item. Examples of artwork for a content item are the album
         cover for a song, or a movie poster for a movie.
        */
        #[method_id(@__retain_semantics Other artwork)]
        pub unsafe fn artwork(&self) -> Option<Id<MPMediaItemArtwork>>;

        #[cfg(feature = "MediaPlayer_MPMediaItemArtwork")]
        /**
          Artwork for this item. Examples of artwork for a content item are the album
         cover for a song, or a movie poster for a movie.
        */
        #[method(setArtwork:)]
        pub unsafe fn setArtwork(&self, artwork: Option<&MPMediaItemArtwork>);

        /**
          Represents the current playback progress of the item.
         0.0 = not watched/listened/viewed, 1.0 = fully watched/listened/viewed
         Default is -1.0 (no progress indicator shown)
        */
        #[method(playbackProgress)]
        pub unsafe fn playbackProgress(&self) -> c_float;

        /**
          Represents the current playback progress of the item.
         0.0 = not watched/listened/viewed, 1.0 = fully watched/listened/viewed
         Default is -1.0 (no progress indicator shown)
        */
        #[method(setPlaybackProgress:)]
        pub unsafe fn setPlaybackProgress(&self, playback_progress: c_float);

        /**
          Represents whether this content item is streaming content, i.e. from the cloud
         where the content is not stored locally.
        */
        #[method(isStreamingContent)]
        pub unsafe fn isStreamingContent(&self) -> bool;

        /**
          Represents whether this content item is streaming content, i.e. from the cloud
         where the content is not stored locally.
        */
        #[method(setStreamingContent:)]
        pub unsafe fn setStreamingContent(&self, streaming_content: bool);

        /**
          Represents whether this content item is explicit content
        */
        #[method(isExplicitContent)]
        pub unsafe fn isExplicitContent(&self) -> bool;

        /**
          Represents whether this content item is explicit content
        */
        #[method(setExplicitContent:)]
        pub unsafe fn setExplicitContent(&self, explicit_content: bool);

        /**
          Represents whether the content item is a container that may contain other
         content items, e.g. an album or a playlist.
        */
        #[method(isContainer)]
        pub unsafe fn isContainer(&self) -> bool;

        /**
          Represents whether the content item is a container that may contain other
         content items, e.g. an album or a playlist.
        */
        #[method(setContainer:)]
        pub unsafe fn setContainer(&self, container: bool);

        /**
          Represents whether the content item is actionable from a playback
         perspective. Albums are playable, for example, because selecting an album
         for playback means the app should play each song in the album in order. An
         example of a content item that may not be playable is a genre, since an app
         experience typically doesn't involve selecting an entire genre for playback.
        */
        #[method(isPlayable)]
        pub unsafe fn isPlayable(&self) -> bool;

        /**
          Represents whether the content item is actionable from a playback
         perspective. Albums are playable, for example, because selecting an album
         for playback means the app should play each song in the album in order. An
         example of a content item that may not be playable is a genre, since an app
         experience typically doesn't involve selecting an entire genre for playback.
        */
        #[method(setPlayable:)]
        pub unsafe fn setPlayable(&self, playable: bool);
    }
);
