import           Control.Lens           ((^.))
import           Control.Monad.IO.Class (liftIO)
import           Web.Pixiv
import           Web.Pixiv.Types.Lens

main :: IO ()
main = do
  let credential = RefreshToken "token"
  result <- runPixivT' credential action
  case result of
    Left err -> print err
    Right x  -> pure x

action :: PixivT IO ()
action = do
  -- gets the details of user <https://www.pixiv.net/users/16731>
  userDetail <- getUserDetail 16731
  liftIO $ print userDetail

  -- gets the details of illustration <https://www.pixiv.net/artworks/80132896>
  illustDetail <- getIllustDetail 80132896
  liftIO $ print illustDetail

  -- gets day ranking illustrations
  -- 1 means the first page of the results
  ranking <- getIllustRanking (Just Day) 1
  liftIO $ print ranking

  -- searches the user who has name "玉之けだま" then gets their first work
  -- (function 'head' is not total, just used for demonstration)
  targetUser <- head <$> searchUser "玉之けだま" Nothing 1
  firstWork <- head <$> getUserIllusts (targetUser ^. user . userId) (Just TypeIllust) 1
  liftIO $ print firstWork
