{-# LANGUAGE OverloadedStrings #-}

module Lib
    (
    ) where

import qualified Data.Ini as Ini
import qualified Data.Text as T
import Data.Text (Text)

data Aws = Aws { access :: Text
               , secret :: Text
               } deriving (Show)

{-
whenRight :: Monad m => Either a b -> (b -> m ()) -> m ()
whenRight (Right r) f = f r
whenRight _ _ = return ()
-}

-- Duh. `whenRight` = `fmap` for (Functor (Either a))
whenRight :: Either a b -> (b -> c) -> Either a c
whenRight (Right r) f = Right (f r)
whenRight (Left l) _ = Left l

findS3cmd :: Text -> Ini.Ini -> Either String Aws
findS3cmd section ini = do
  case Ini.lookupValue section "access_key" ini of
    Left err -> Left err
    Right acc -> case Ini.lookupValue section "secret_key" ini of
      Left err -> Left err
      Right sec -> Right Aws { access = acc, secret = sec }

parseSection :: Text -> String -> [Text]
parseSection initext section = do
  case Ini.parseIni initext of
    Left err -> []
    Right ini -> Ini.sections ini
